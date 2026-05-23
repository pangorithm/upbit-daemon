pub mod init;
pub mod markets;
pub mod partition;

use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

pub async fn create_pool(database_url: &str) -> Result<PgPool, crate::error::AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    info!("Database connection established");

    Ok(pool)
}
