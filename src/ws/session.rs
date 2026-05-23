use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use sqlx::PgPool;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};

use crate::api::websocket::WebSocketClient;
use crate::collector;
use crate::config;
use crate::cron;

static SUBSCRIBED_UNITS: OnceLock<std::sync::Mutex<HashSet<String>>> = OnceLock::new();

pub(super) fn get_subscribed_units() -> &'static std::sync::Mutex<HashSet<String>> {
    SUBSCRIBED_UNITS.get_or_init(|| std::sync::Mutex::new(HashSet::new()))
}

fn matches_prefix(market: &str, prefix: &Option<String>) -> bool {
    match prefix {
        Some(p) => market.starts_with(p),
        None => true,
    }
}

pub(super) async fn find_and_subscribe_new(
    pool: &PgPool,
    ws: &WebSocketClient,
    config: &config::Config,
    subscribed: &HashSet<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let db_markets = crate::db::markets::fetch_all_markets(pool).await?;

    let new_markets: Vec<String> = db_markets
        .iter()
        .filter(|m| !subscribed.contains(m.as_str()))
        .cloned()
        .collect();

    let filtered: Vec<String> = new_markets
        .into_iter()
        .filter(|m| matches_prefix(&m, &config.candle.market_prefix))
        .collect();

    for market in &filtered {
        if let Err(e) = collector::subscriptions::subscribe_new_market(ws, config, market).await {
            error!("Failed to subscribe new market {}: {}", market, e);
        }
    }

    Ok(filtered)
}

async fn refresh_candle_subscriptions(
    pool: &PgPool,
    ws: &WebSocketClient,
    config: &config::Config,
    subscribed_units: &'static std::sync::Mutex<HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscribed: HashSet<String> = subscribed_units.lock().unwrap().iter().cloned().collect();
    let filtered = find_and_subscribe_new(pool, ws, config, &subscribed).await?;
    if !filtered.is_empty() {
        let mut lock = subscribed_units.lock().unwrap();
        for market in &filtered {
            lock.insert(market.clone());
        }
    }
    info!("Subscribed to {} new markets", filtered.len());
    Ok(())
}

pub async fn run_ws_session(
    ws_pool: PgPool,
    ws_config: config::Config,
    access_key: Option<String>,
    secret_key: Option<String>,
    label: &str,
) {
    let ws = match WebSocketClient::connect(
        ws_config.url.ws.clone(), access_key.clone(), secret_key.clone(),
    ).await {
        Ok(client) => client,
        Err(e) => {
            error!("WebSocket connect failed ({}): {}", label, e);
            return;
        }
    };
    info!("WebSocket connected ({})", label);

    let subscribed_units = get_subscribed_units();
    info!("Previously subscribed units: {:?}", subscribed_units.lock().unwrap());

    let msg_count = Arc::new(AtomicU64::new(0));
    {
        let mc = Arc::clone(&msg_count);
        tokio::spawn(async move {
            let mut timer = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                timer.tick().await;
                let count = mc.fetch_and(0, Ordering::SeqCst);
                if count > 0 {
                    info!(received_msgs = count, "WebSocket message stats");
                }
            }
        });
    }

   let new_units: Vec<&String> = ws_config.candle.units.iter()
        .filter(|u| !subscribed_units.lock().unwrap().contains(u.as_str()))
        .collect();

    if new_units.is_empty() {
        info!("All units already subscribed, skipping initial subscription");
    } else {
        match collector::subscriptions::subscribe_markets(&ws_pool, &ws, &ws_config, &new_units).await {
            Ok(subscribed) => {
                info!("Successfully subscribed units: {:?}", subscribed);
                let mut lock = subscribed_units.lock().unwrap();
                for unit in subscribed {
                    lock.insert(unit);
                }
            }
            Err(e) => {
                error!("Failed to subscribe markets: {}", e);
            }
        }
    }

     let (refresh_cancel_tx, mut refresh_cancel_rx) = watch::channel(());
    let ws_clone = ws.clone();
    let pool_clone = ws_pool.clone();
    let config_clone = ws_config.clone();
    let refresh_handle = tokio::spawn(async move {
        loop {
            let next = cron::interval::next_cron_instant(
                config_clone.cron.subscribe.as_deref(),
                tokio::time::Instant::now() + std::time::Duration::from_secs(600),
            );
            tokio::select! {
                _ = tokio::time::sleep_until(next) => {
                    if let Err(e) = refresh_candle_subscriptions(&pool_clone, &ws_clone, &config_clone, &subscribed_units).await {
                        error!("Subscription refresh failed: {}", e);
                    }
                }
                _ = refresh_cancel_rx.changed() => {
                    break;
                }
            }
        }
    });

     let (keepalive_tx, mut keepalive_rx) = mpsc::channel::<()>(1);
    let keepalive_handle = ws.keepalive(keepalive_tx.clone());

     loop {
        tokio::select! {
            biased;

            result = ws.recv() => {
                match result {
                    Some(Ok(msg)) => {
                        match msg {
                            Message::Text(text) => {
                                debug!(text = %text, "WebSocket message received");
                                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                                    if parsed.as_array().map_or(false, |arr| {
                                        arr.iter().any(|item| {
                                            item.get("type").and_then(|t| t.as_str())
                                                .map_or(false, |t| t == "LIST_SUBSCRIPTIONS")
                                        })
                                    }) {
                                        if let Err(e) = super::handler::handle_subscription_update(&ws_pool, &ws, &ws_config, &parsed, &subscribed_units).await {
                                            error!("Failed to handle subscription update: {}", e);
                                        }
                                    } else {
                                        msg_count.fetch_add(1, Ordering::SeqCst);
                                        if let Err(e) = collector::parsers::handle_message(&ws_pool, &parsed).await {
                                            error!("Failed to handle message: {}", e);
                                        }
                                    }
                                }
                            }
                            Message::Ping(_) | Message::Pong(_) => continue,
                            Message::Close(_) => {
                                warn!("WebSocket closed ({}), will reconnect...", label);
                                break;
                            }
                            _ => continue,
                        }
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error ({}): {}", label, e);
                        break;
                    }
                    None => {
                        warn!("WebSocket stream ended ({}), will reconnect...", label);
                        break;
                    }
                }
            }

            _ = keepalive_rx.recv() => {
                warn!("Keepalive failed ({}), reconnecting...", label);
                break;
            }
        }
    }

    drop(keepalive_tx);
    let _ = keepalive_handle.await;

    refresh_cancel_tx.send(()).ok();
    let _ = refresh_handle.await;
}
