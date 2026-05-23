use sqlx::PgPool;
use tracing::{error, info};
use crate::config::Config;
use crate::api::rest::RestClient;

pub async fn run_market_refresh(pool: &PgPool, rest: &RestClient, config: &Config) {
    info!("Starting market refresh (cron.market interval)");

    loop {
        let next = crate::cron::interval::next_cron_instant(
            config.cron.market.as_deref(),
            tokio::time::Instant::now() + std::time::Duration::from_secs(600),
        );
        tokio::time::sleep_until(next).await;
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
