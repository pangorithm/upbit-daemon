use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};

use serde_json::json;
use sqlx::PgPool;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};

use super::client::WebSocketClient;
use crate::collector;
use crate::config;
use crate::cron;

static SUBSCRIBED_MARKETS: OnceLock<std::sync::Mutex<HashSet<String>>> = OnceLock::new();

pub(super) fn get_subscribed_markets() -> &'static std::sync::Mutex<HashSet<String>> {
    SUBSCRIBED_MARKETS.get_or_init(|| std::sync::Mutex::new(HashSet::new()))
}

async fn subscribe_markets(
    ws: &WebSocketClient,
    type_name: &str,
    markets: &[String],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if markets.is_empty() {
        return Ok(());
    }

    let ws_type = match type_name {
        "1s" => "candle.1s",
        "ticker" => "ticker",
        "trade" => "trade",
        "orderbook" => "orderbook",
        _ => type_name,
    };

    for chunk in markets.chunks(10) {
        let msg = json!([
            {"ticket": uuid::Uuid::new_v4().to_string()},
            {
                "type": ws_type,
                "codes": chunk
            },
            {"format": "DEFAULT"}
        ]);
        ws.send(Message::Text(msg.to_string().into())).await?;
    }
    Ok(())
}

async fn subscribe_new_markets(
    ws: &WebSocketClient,
    type_name: &str,
    markets: &[String],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if markets.is_empty() {
        return Ok(());
    }

    let ws_type = match type_name {
        "1s" => "candle.1s",
        "ticker" => "ticker",
        "trade" => "trade",
        "orderbook" => "orderbook",
        _ => type_name,
    };

    let msg = json!([
        {"ticket": uuid::Uuid::new_v4().to_string()},
        {
            "type": ws_type,
            "codes": markets
        },
        {"format": "DEFAULT"}
    ]);
    ws.send(Message::Text(msg.to_string().into())).await?;
    Ok(())
}

pub(super) fn find_and_subscribe_new(
    _ws: &WebSocketClient,
    _type_name: &str,
    config: &config::Config,
    subscribed_list: &[String],
) -> Vec<String> {
    let prefix = &config.candle.market_prefix;
    subscribed_list
        .iter()
        .filter(|m| matches_prefix(m, prefix))
        .cloned()
        .collect()
}

fn matches_prefix(market: &str, prefix: &Option<String>) -> bool {
    match prefix {
        Some(p) => market.starts_with(p),
        None => true,
    }
}

async fn refresh_subscriptions(
    ws: &WebSocketClient,
    config: &config::Config,
    subscribed_list: &[String],
    type_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let prefix = &config.candle.market_prefix;
    let markets_to_subscribe: Vec<String> = subscribed_list
        .iter()
        .filter(|m| matches_prefix(m, prefix))
        .cloned()
        .collect();

    for market in &markets_to_subscribe {
        subscribe_new_markets(ws, type_name, &[market.clone()]).await.ok();
    }

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

    let subscribed_markets = get_subscribed_markets();

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

subscribe_candle(&ws, &ws_config).await;
    subscribe_ticker(&ws, &ws_config).await;
    subscribe_trade(&ws, &ws_config).await;
    subscribe_orderbook(&ws, &ws_config).await;

    let (refresh_cancel_tx, mut refresh_cancel_rx) = watch::channel(());
    let ws_clone = ws.clone();
    let config_clone = ws_config.clone();
    let refresh_handle = tokio::spawn(async move {
        loop {
            let next = cron::interval::next_cron_instant(
                config_clone.cron.subscribe.as_deref(),
                tokio::time::Instant::now() + std::time::Duration::from_secs(600),
            );
            tokio::select! {
                _ = tokio::time::sleep_until(next) => {
                    if let Err(e) = refresh_all_subscriptions(&ws_clone, &config_clone).await {
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
                                        if let Err(e) = super::handler::handle_subscription_update(&ws_pool, &ws, &ws_config, &parsed, &subscribed_markets).await {
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

async fn subscribe_candle(ws: &WebSocketClient, config: &config::Config) {
    if config.subscribe.candle.is_empty() {
        return;
    }
    info!("Subscribing to candle for {} markets", config.subscribe.candle.len());
    for chunk in config.subscribe.candle.iter() {
        let msg = json!([
            {"ticket": uuid::Uuid::new_v4().to_string()},
            {
                "type": "candle.1s",
                "codes": [chunk.as_str()]
            },
            {"format": "DEFAULT"}
        ]);
        if let Err(e) = ws.send(Message::Text(msg.to_string().into())).await {
            error!("Failed to subscribe candle for {}: {}", chunk, e);
        } else {
            let mut lock = get_subscribed_markets().lock().unwrap();
            lock.insert(chunk.clone());
        }
    }
}

async fn subscribe_ticker(ws: &WebSocketClient, config: &config::Config) {
    if config.subscribe.ticker.is_empty() {
        return;
    }
    info!("Subscribing to ticker for {} markets", config.subscribe.ticker.len());
    subscribe_markets(ws, "ticker", &config.subscribe.ticker).await.ok();
    for market in &config.subscribe.ticker {
        if matches_prefix(market, &config.candle.market_prefix) {
            let mut lock = get_subscribed_markets().lock().unwrap();
            lock.insert(market.clone());
        }
    }
}

async fn subscribe_trade(ws: &WebSocketClient, config: &config::Config) {
    if config.subscribe.trade.is_empty() {
        return;
    }
    info!("Subscribing to trade for {} markets", config.subscribe.trade.len());
    subscribe_markets(ws, "trade", &config.subscribe.trade).await.ok();
    for market in &config.subscribe.trade {
        if matches_prefix(market, &config.candle.market_prefix) {
            let mut lock = get_subscribed_markets().lock().unwrap();
            lock.insert(market.clone());
        }
    }
}

async fn subscribe_orderbook(ws: &WebSocketClient, config: &config::Config) {
    if config.subscribe.orderbook.is_empty() {
        return;
    }
    info!("Subscribing to orderbook for {} markets", config.subscribe.orderbook.len());
    subscribe_markets(ws, "orderbook", &config.subscribe.orderbook).await.ok();
    for market in &config.subscribe.orderbook {
        if matches_prefix(market, &config.candle.market_prefix) {
            let mut lock = get_subscribed_markets().lock().unwrap();
            lock.insert(market.clone());
        }
    }
}

async fn refresh_all_subscriptions(
    ws: &WebSocketClient,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    refresh_subscriptions(ws, config, &config.subscribe.ticker, "ticker").await?;
    refresh_subscriptions(ws, config, &config.subscribe.trade, "trade").await?;
    refresh_subscriptions(ws, config, &config.subscribe.orderbook, "orderbook").await?;
    Ok(())
}
