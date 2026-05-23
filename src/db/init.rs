use sqlx::PgPool;
use tracing::{error, info, warn};
use crate::config::Config;
use crate::db::partition::ensure_partitions;

/// Initialize database: ensure tables exist and create all partitions (gap-filling + future)
pub async fn init_database(pool: &PgPool, config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ensure_tables_exist(pool).await?;
    if let Err(e) = ensure_partitions(pool, config).await {
        error!(error = %e, "Failed to ensure partitions");
    }
    info!("Database initialization completed");
    Ok(())
}

async fn ensure_tables_exist(pool: &PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tables = ["markets", "tickers", "trades", "candles_seconds", "candles_minutes", "candles_days", "orderbooks"];
    let existing = sqlx::query_scalar::<_, String>(
        r#"SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE' ORDER BY table_name"#
    ).fetch_all(pool).await?;
    let existing_set: std::collections::HashSet<&str> = existing.iter().map(|s| s.as_str()).collect();

    let missing: Vec<&str> = tables.iter().filter(|t| !existing_set.contains(**t)).copied().collect();
    if missing.is_empty() {
        info!("All tables exist");
        return Ok(());
    }

    warn!("Missing tables: {:?}", missing);

    for table in &missing {
        let sql = std::fs::read_to_string("migrations/001_initial.sql")?;
        for statement in sql.split(';') {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                if stmt.contains(format!("CREATE TABLE {}", table).as_str()) {
                    if let Err(e) = sqlx::query(stmt).execute(pool).await {
                        error!(table = %table, error = %e, "Failed to create table");
                    } else {
                        info!("Created table: {}", table);
                    }
                }
            }
        }
    }
    info!("Database initialization completed");
    Ok(())
}
