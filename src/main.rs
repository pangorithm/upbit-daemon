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

/// Run a single WebSocket session: connect → subscribe → handle messages
/// Returns on disconnect; caller handles reconnection with backoff
async fn run_ws_session(
    ws_pool: sqlx::PgPool,
    ws_config: &config::Config,
    access_key: Option<String>,
    secret_key: Option<String>,
    label: &str,
) {
    let mut ws = match api::websocket::WebSocketClient::connect(
        ws_config.url.ws.clone(), access_key.clone(), secret_key.clone(),
    ).await {
        Ok(client) => client,
        Err(e) => {
            error!("WebSocket connect failed ({}): {}", label, e);
            return;
        }
    };
    info!("WebSocket connected ({})", label);

    let markets_rows = match sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(&ws_pool).await {
        Ok(rows) => rows,
        Err(e) => {
            error!("Failed to fetch markets for {}: {}", label, e);
            return;
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
            error!("Failed to {} candle subscription (unit={}): {}", label, unit, e);
        } else {
            info!("{}: subscribed to {} markets for candle.{} units", label, markets.len(), unit);
        }
    }

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
                    warn!("WebSocket closed ({}), will reconnect...", label);
                    return;
                }
                _ => continue,
            },
            Some(Err(e)) => {
                error!("WebSocket error ({}): {}", label, e);
                return;
            }
            None => {
                warn!("WebSocket stream ended ({}), will reconnect...", label);
                return;
            }
        }
    }
}

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

    // 5. WebSocket session: connect → subscribe → handle with auto-reconnect
    //    On disconnect: retry with exponential backoff (1s→30s max)
    let ws_pool = pool.clone();
    let ws_config = config.clone();
    let ws_handle = tokio::spawn(async move {
        let mut reconnect_delay_secs: u64 = 1;
        let max_reconnect_delay_secs: u64 = 30;
        loop {
            run_ws_session(ws_pool.clone(), &ws_config, access_key.clone(), secret_key.clone(), "WS").await;
            tokio::time::sleep(std::time::Duration::from_secs(reconnect_delay_secs)).await;
            reconnect_delay_secs = (reconnect_delay_secs * 2).min(max_reconnect_delay_secs);
        }
    });

    // 6. Partition cron: create future partitions, delete expired partitions (24h cycle)
    let cron_config = config.clone();
    tokio::spawn(async move {
        cron::partition_schedule::run_partition_schedule(&pool, &cron_config).await;
    });

    // Wait for shutdown signal (Ctrl+C)
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");
    drop(ws_handle);
    Ok(())
}
