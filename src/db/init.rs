use crate::config::Config;
use crate::db::partition::ensure_partitions;
use sqlx::PgPool;
use tracing::{error, info};

/// Initialize database: ensure tables exist and create all partitions (gap-filling + future)
pub async fn init_database(
    pool: &PgPool,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ensure_tables_exist(pool).await;
    if let Err(e) = ensure_partitions(pool, config).await {
        error!(error = %e, "Failed to ensure partitions");
    }
    info!("Database initialization completed");
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/expected_tables.rs"));

async fn ensure_tables_exist(pool: &PgPool) {
    let existing = sqlx::query_scalar::<_, String>(
        r#"SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE' ORDER BY table_name"#
    ).fetch_all(pool).await
        .expect("Failed to query tables");
    let existing_set: std::collections::HashSet<&str> =
        existing.iter().map(|s| s.as_str()).collect();

    let missing: Vec<&str> = EXPECTED_TABLES
        .iter()
        .filter(|t| !existing_set.contains(*t))
        .copied()
        .collect();
    if missing.is_empty() {
        info!("All tables exist");
        return;
    }

    panic!(
        "Missing tables: {:?}. Run `cargo sqlx migrate run` to create them.",
        missing
    );
}
