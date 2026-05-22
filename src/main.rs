mod api;
mod collector;
mod config;
mod cron;
mod db;
mod error;

use clap::Parser;
use sqlx::Row;
use std::collections::HashSet;
use std::sync::OnceLock;
use tokio::sync::mpsc;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info, warn};

/// Run a single WebSocket session: connect → subscribe → handle messages
/// Returns on disconnect; caller handles reconnection with backoff
async fn run_ws_session(
    ws_pool: sqlx::PgPool,
    ws_config: config::Config,
    rest: crate::api::rest::RestClient,
    access_key: Option<String>,
    secret_key: Option<String>,
    label: &str,
) {
    let ws = match api::websocket::WebSocketClient::connect(
        ws_config.url.ws.clone(), access_key.clone(), secret_key.clone(),
    ).await {
        Ok(client) => client,
        Err(e) => {
            error!("WebSocket connect failed ({}): {}", label, e);
            return;
        }
    };
    info!("WebSocket connected ({})", label);

    if let Err(e) = collector::subscriptions::subscribe_markets(&ws_pool, &ws, &ws_config).await {
        error!("Failed to subscribe markets: {}", e);
        return;
    }

    if let Err(e) = collector::candles::fill_all_candle_gaps(&ws_pool, &rest, &ws_config).await {
        error!("Gap-filling failed: {}", e);
    }

    let ws_clone = ws.clone();
    let pool_clone = ws_pool.clone();
    let config_clone = ws_config.clone();
    tokio::spawn(async move {
        let mut timer = interval(std::time::Duration::from_secs(10 * 60));
        loop {
            timer.tick().await;
            if let Err(e) = refresh_candle_subscriptions(&pool_clone, &ws_clone, &config_clone).await {
                error!("Subscription refresh failed: {}", e);
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
                    Some(Ok(msg)) => match msg {
                        Message::Text(text) => {
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                                if parsed.as_array().map_or(false, |arr| {
                                    arr.iter().any(|item| {
                                        item.get("type").and_then(|t| t.as_str())
                                            .map_or(false, |t| t == "LIST_SUBSCRIPTIONS")
                                    })
                                }) {
                                    if let Err(e) = handle_subscription_update(&ws_pool, &ws, &ws_config, &parsed).await {
                                        error!("Failed to handle subscription update: {}", e);
                                    }
                                } else if let Err(e) = collector::parsers::handle_message(&ws_pool, &parsed).await {
                                    error!("Failed to handle message: {}", e);
                                }
                            }
                        }
                        Message::Ping(_) | Message::Pong(_) => continue,
                        Message::Close(_) => {
                            warn!("WebSocket closed ({}), will reconnect...", label);
                            break;
                        }
                        _ => continue,
                    },
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
}

async fn refresh_candle_subscriptions(
    pool: &sqlx::PgPool,
    ws: &api::websocket::WebSocketClient,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    static SUBSCRIBED: OnceLock<std::sync::Mutex<HashSet<String>>> = OnceLock::new();
    let subscribed = SUBSCRIBED.get_or_init(|| std::sync::Mutex::new(HashSet::new()));

    let db_markets_rows = sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(pool).await?;
    let db_markets: Vec<String> = db_markets_rows.iter()
        .map(|r: &sqlx::postgres::PgRow| r.get("market"))
        .collect();

    if db_markets.is_empty() {
        return Ok(());
    }

    let new_markets: Vec<String> = {
        let lock = subscribed.lock().unwrap();
        db_markets.iter()
            .filter(|m| !lock.contains(m.as_str()))
            .cloned()
            .collect()
    };

    if new_markets.is_empty() {
        return Ok(());
    }

    for market in &new_markets {
        if let Err(e) = collector::subscriptions::subscribe_new_market(ws, config, market).await {
            error!("Failed to subscribe new market {}: {}", market, e);
        }
    }

    {
        let mut lock = subscribed.lock().unwrap();
        for market in &new_markets {
            lock.insert(market.clone());
        }
    }

    info!("Subscribed to {} new markets", new_markets.len());
    Ok(())
}

async fn handle_subscription_update(
    pool: &sqlx::PgPool,
    ws: &api::websocket::WebSocketClient,
    config: &config::Config,
    msg: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscribed_codes = msg.as_array().and_then(|arr| {
        arr.iter().find_map(|item| {
            item.get("codes").and_then(|codes| codes.as_array())
                .map(|c| c.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
        })
    });

    let db_markets_rows = sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(pool).await?;
    let db_markets: Vec<String> = db_markets_rows.iter()
        .map(|r: &sqlx::postgres::PgRow| r.get("market"))
        .collect();

    let subscribed = match subscribed_codes {
        Some(codes) => codes.into_iter().collect::<std::collections::HashSet<_>>(),
        None => std::collections::HashSet::new(),
    };

    let new_markets: Vec<&str> = db_markets.iter()
        .filter(|m| !subscribed.contains(m.as_str()))
        .map(|s| s.as_str())
        .collect();

    if !new_markets.is_empty() {
        info!("Found {} new markets, subscribing...", new_markets.len());
        for market in new_markets {
            if let Err(e) = collector::subscriptions::subscribe_new_market(ws, config, market).await {
                error!("Failed to subscribe new market {}: {}", market, e);
            }
        }
    }

    Ok(())
}

/// Upbit daemon: WebSocket subscription + gap-filling + cron partition management
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    #[cfg(feature = "dev")]
    dotenvy::dotenv().ok();

    let cli = config::Cli::parse();
    info!("DATABASE_URL: set");

    let config_str = std::fs::read_to_string(&cli.config_path)?;
    let config: config::Config = serde_yaml::from_str(&config_str)?;

    info!("REST_URL: {}", config.url.rest);
    info!("WS_URL: {}", config.url.ws);
    info!("candle_units: {:?}", config.candle.units);
    info!("candle_count: {}", config.candle.count);
    info!("rate_limit: {} calls/sec", config.rate_limit.api_calls_per_second);
    info!("partition: create={}, retain_days={}, retain_months={}",
        config.partition.create, config.partition.retain_days, config.partition.retain_months);

    let access_key = if cli.access_key.is_empty() { None } else { Some(cli.access_key.clone()) };
    let secret_key = if cli.secret_key.is_empty() { None } else { Some(cli.secret_key.clone()) };

    let pool = db::create_pool(&cli.database_url).await?;
    info!("Database connected");

    let rest = api::rest::RestClient::new(&config.url.rest, access_key.clone(), secret_key.clone());
    info!("REST client created");

    if let Err(e) = db::init::init_database(&pool).await {
        error!("Database initialization failed: {}", e);
    }

    if let Err(e) = db::partition::create_future_partitions(&pool, &config).await {
        error!("Failed to create future partitions: {}", e);
    }

    if let Err(e) = api::quotation::market::fetch_and_upsert_markets(&pool, &rest).await {
        error!("Failed to fetch markets: {}", e);
    }

    let ws_pool = pool.clone();
    let ws_config = config.clone();
    let ws_rest = rest.clone();
    let ws_rest_inner = ws_rest.clone();
    let ws_rest_market = ws_rest.clone();
    let ws_handle = tokio::spawn(async move {
        let mut reconnect_delay_secs: u64 = 1;
        let max_reconnect_delay_secs: u64 = 30;
        loop {
            run_ws_session(ws_pool.clone(), ws_config.clone(), ws_rest_inner.clone(),
                           access_key.clone(), secret_key.clone(), "WS").await;
            tokio::time::sleep(std::time::Duration::from_secs(reconnect_delay_secs)).await;
            reconnect_delay_secs = (reconnect_delay_secs * 2).min(max_reconnect_delay_secs);
        }
    });

    let cron_config = config.clone();
    let cron_pool = pool.clone();
    let cron_pool_inner = cron_pool.clone();
    let cron_pool_market = cron_pool.clone();
    let cron_handle = tokio::spawn(async move {
        cron::partition_schedule::run_partition_schedule(&cron_pool_inner, &cron_config).await;
    });

    let market_config = config.clone();
    let market_pool = cron_pool_market.clone();
    let market_rest = ws_rest_market.clone();
    tokio::spawn(async move {
        cron::market_refresh::run_market_refresh(&market_pool, &market_rest, &market_config).await;
    });

    let _ = cron_handle.await;

    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");
    drop(ws_handle);
    Ok(())
}
