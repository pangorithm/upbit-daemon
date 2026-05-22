use sqlx::PgPool;
use tracing::{error, info};
use crate::config::ApiConfig;
use crate::db::partition::create_future_partitions;

pub async fn run_partition_schedule(pool: &PgPool, config: &ApiConfig) -> ! {
    info!("Starting partition schedule");

    if let Err(e) = run_once(pool, config).await {
        error!("Partition schedule failed: {}", e);
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(24 * 60 * 60)).await;
        if let Err(e) = run_once(pool, config).await {
            error!("Partition schedule failed: {}", e);
        }
    }
}

async fn run_once(pool: &PgPool, config: &ApiConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Err(e) = create_future_partitions(pool, config).await {
        error!("Failed to create future partitions: {}", e);
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
