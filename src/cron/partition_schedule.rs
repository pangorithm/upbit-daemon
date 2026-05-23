use sqlx::PgPool;
use tracing::{error, info};
use crate::config::Config;
use crate::db::partition::ensure_partitions;

pub async fn run_partition_schedule(pool: &PgPool, config: &Config) -> ! {
    info!("Starting partition schedule (cron.partition)");

    loop {
        let next = crate::cron::interval::next_cron_instant(
            config.cron.partition.as_deref(),
            tokio::time::Instant::now() + std::time::Duration::from_secs(600),
        );
        tokio::time::sleep_until(next).await;
        if let Err(e) = run_once(pool, config).await {
            error!("Partition schedule failed: {}", e);
        }
    }
}

async fn run_once(pool: &PgPool, config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Err(e) = ensure_partitions(pool, config).await {
        error!("Failed to ensure partitions: {}", e);
    }
    if let Err(e) = crate::cron::partition_delete::delete_daily_partitions(pool, config).await {
        error!("Failed to delete daily partitions: {}", e);
    }
    if let Err(e) = crate::cron::partition_delete::delete_monthly_partitions(pool, config).await {
        error!("Failed to delete monthly partitions: {}", e);
    }
    info!("Partition schedule completed");
    Ok(())
}
