mod api;
mod collector;
mod config;
mod cron;
mod db;
mod error;

use clap::Parser;
use sqlx::Row;
use tracing::{error, info, warn};
use tokio_tungstenite::tungstenite::Message;

/// Upbit daemon: WebSocket subscription + gap-filling + cron partition management
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
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

    // 1. Database connection
    let pool = db::create_pool(&cli.database_url).await?;
    info!("Database connected");

    let rest = api::rest::RestClient::new(&config.url.rest, access_key.clone(), secret_key.clone());
    info!("REST client created");

    // 2. Initialize database: create tables if missing, fill partition gaps
    if let Err(e) = db::init::init_database(&pool).await {
        error!("Database initialization failed: {}", e);
    }

    // 3. Pre-create future partitions (avoid cold starts when cron is down)
    if let Err(e) = db::partition::create_future_partitions(&pool, &config).await {
        error!("Failed to create future partitions: {}", e);
    }

    // 4. Fetch trading pairs from Upbit REST API
    if let Err(e) = api::quotation::market::fetch_and_upsert_markets(&pool, &rest).await {
        error!("Failed to fetch markets: {}", e);
    }

    // 5. WebSocket connection (JWT auth for public endpoint)
    let mut ws = api::websocket::WebSocketClient::connect(config.url.ws.clone(), access_key.clone(), secret_key.clone()).await?;
    info!("WebSocket connected");

    // 6. Candle subscription: subscribe all markets for each configured candle unit
    let markets_rows = sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(&pool).await?;
    let markets: Vec<String> = markets_rows.iter().map(|r: &sqlx::postgres::PgRow| r.get("market")).collect();

    if !markets.is_empty() {
        // Subscribe to all candle units (e.g. 1m, 10m, 60m)
        for &unit in &config.candle.units {
            let candle_msg = serde_json::json!([
                {
                    "format": "DEFAULT",
                    "type": format!("candle.{}", unit),
                    "codes": markets
                }
            ]);
            if let Err(e) = ws.send(Message::Text(candle_msg.to_string().into())).await {
                error!("Failed to send candle subscription (unit={}): {}", unit, e);
            } else {
                info!("Subscribed to {} markets for candle.{} units", markets.len(), unit);
            }
        }

        // Gap-fill: for each market + unit, fill missing candles from REST API
        for market in &markets {
            for &unit in &config.candle.units {
                if let Err(e) = collector::candles::fill_candle_gap(&pool, &rest, &config, market, unit).await {
                    error!(market, unit, "Gap fill failed: {}", e);
                }
            }
        }
    }

    // 7. WebSocket message handler with reconnection
    //    On disconnect: reconnect, resubscribe, resume (exponential backoff: 1s→2s→4s→...→max 30s)
    let ws_pool = pool.clone();
    let ws_config = config.clone();
    let _ws_handle = tokio::spawn(async move {
        let mut reconnect_delay_secs: u64 = 1;
        let max_reconnect_delay_secs: u64 = 30;
        loop {
            // Connect
            let mut ws = match api::websocket::WebSocketClient::connect(
                ws_config.url.ws.clone(), access_key.clone(), secret_key.clone(),
            ).await {
                Ok(client) => client,
                Err(e) => {
                    error!("WebSocket connect failed (delay={}s): {}", reconnect_delay_secs, e);
                    tokio::time::sleep(std::time::Duration::from_secs(reconnect_delay_secs)).await;
                    reconnect_delay_secs = (reconnect_delay_secs * 2).min(max_reconnect_delay_secs);
                    continue;
                }
            };
            info!("WebSocket connected");

            // Resubscribe all markets for all candle units
            reconnect_delay_secs = 1; // reset backoff on successful connect
            let markets_rows = match sqlx::query(
                r#"SELECT market FROM markets ORDER BY market"#
            ).fetch_all(&ws_pool).await {
                Ok(rows) => rows,
                Err(e) => {
                    error!("Failed to fetch markets for re-subscription: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(reconnect_delay_secs)).await;
                    reconnect_delay_secs = (reconnect_delay_secs * 2).min(max_reconnect_delay_secs);
                    continue;
                }
            };
            let markets: Vec<String> = markets_rows.iter().map(|r: &sqlx::postgres::PgRow| r.get("market")).collect();

            for &unit in &ws_config.candle.units {
                let candle_msg = serde_json::json!([
                    {
                        "format": "DEFAULT",
                        "type": format!("candle.{}", unit),
                        "codes": markets.clone()
                    }
                ]);
                if let Err(e) = ws.send(Message::Text(candle_msg.to_string().into())).await {
                    error!("Failed to resubscribe candle (unit={}): {}", unit, e);
                } else {
                    info!("Re-subscribed to {} markets for candle.{} units", markets.len(), unit);
                }
            }

            // Handle messages until disconnect
            loop {
                match ws.recv().await {
                    Some(Ok(msg)) => match msg {
                        Message::Text(text) => {
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                                if let Err(e) = collector::parsers::handle_message(&ws_pool, &parsed).await {
                                    error!("Failed to handle message: {}", e);
                                }
                            }
                        }
                        Message::Ping(_) | Message::Pong(_) => continue,
                        Message::Close(_) => {
                            warn!("WebSocket closed, will reconnect...");
                            break;
                        }
                        _ => continue,
                    },
                    Some(Err(e)) => {
                        error!("WebSocket error, will reconnect: {}", e);
                        break;
                    }
                    None => {
                        warn!("WebSocket stream ended, will reconnect...");
                        break;
                    }
                }
            }
        }
    });

    // 8. Partition cron: create future partitions, delete expired partitions (24h cycle)
    let cron_config = config.clone();
    tokio::spawn(async move {
        cron::partition_schedule::run_partition_schedule(&pool, &cron_config).await;
    });

    // Wait for shutdown signal (Ctrl+C)
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");
    drop(_ws_handle);
    Ok(())
}
