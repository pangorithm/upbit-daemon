use sqlx::PgPool;
use tokio::time::interval;
use tracing::{error, info};
use crate::config::Config;
use crate::api::rest::RestClient;

pub async fn run_market_refresh(pool: &PgPool, rest: &RestClient, config: &Config) {
    info!("Starting market refresh");

    let interval_duration = crate::cron::interval::cron_expression_to_interval(
        config.cron.market.as_deref().unwrap_or("*/10 * * * *"),
    )
    .unwrap_or(std::time::Duration::from_secs(600));

    let ticker = interval(interval_duration);
    tokio::pin!(ticker);

    loop {
        ticker.tick().await;
        if let Err(e) = run_once(pool, rest, config).await {
            error!("Market refresh failed: {}", e);
        }
    }
}

async fn run_once(pool: &PgPool, rest: &RestClient, config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Running market refresh");

    if let Err(e) = crate::api::quotation::market::fetch_and_upsert_markets(pool, rest, &config.candle.market_prefix).await {
        error!("Failed to fetch markets: {}", e);
        return Err(e);
    }

    info!("Market refresh completed");
    Ok(())
}
