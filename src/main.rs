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
    let api_config: config::ApiConfig = serde_yaml::from_str(&config_str)?;

    info!("REST_URL: {}", api_config.rest_url);
    info!("WS_URL: {}", api_config.ws_url);
    info!("candle_unit: {}", api_config.candle_unit);

    let access_key = if cli.access_key.is_empty() { None } else { Some(cli.access_key.clone()) };
    let secret_key = if cli.secret_key.is_empty() { None } else { Some(cli.secret_key.clone()) };

    let pool = db::create_pool(&cli.database_url).await?;
    info!("Database connected");

    let rest = api::rest::RestClient::new(&api_config.rest_url, access_key.clone(), secret_key.clone());
    info!("REST client created");

    if let Err(e) = db::init::init_database(&pool).await {
        error!("Database initialization failed: {}", e);
    }

    if let Err(e) = db::partition::create_future_partitions(&pool, &api_config).await {
        error!("Failed to create future partitions: {}", e);
    }

    if let Err(e) = api::quotation::market::fetch_and_upsert_markets(&pool, &rest).await {
        error!("Failed to fetch markets: {}", e);
    }

    let mut ws = api::websocket::WebSocketClient::connect(api_config.ws_url.clone(), access_key.clone(), secret_key.clone()).await?;
    info!("WebSocket connected");

    let markets_rows = sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(&pool).await?;
    let markets: Vec<String> = markets_rows.iter().map(|r: &sqlx::postgres::PgRow| r.get("market")).collect();

    if !markets.is_empty() {
        info!("Subscribing to {} markets for 1-min candles", markets.len());
        let candle_msg = serde_json::json!([
            {
                "format": "DEFAULT",
                "type": "tick",
                "codes": markets
            }
        ]);
        if let Err(e) = ws.send(Message::Text(candle_msg.to_string().into())).await {
            error!("Failed to send candle subscription: {}", e);
        }
    }

    let ws_pool = pool.clone();
    let ws_handle = tokio::spawn(async move {
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
                        warn!("WebSocket closed, reconnecting...");
                        break;
                    }
                    _ => continue,
                },
                Some(Err(e)) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                None => {
                    warn!("WebSocket stream ended");
                    break;
                }
            }
        }
    });

    let cron_pool = pool.clone();
    let cron_config = api_config.clone();
    tokio::spawn(async move {
        cron::partition_schedule::run_partition_schedule(&cron_pool, &cron_config).await;
    });

    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");
    drop(ws_handle);
    Ok(())
}
