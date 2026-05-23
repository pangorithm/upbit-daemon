use crate::config::Config;
use crate::db::partition::ensure_partitions;
use serde::Deserialize;
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

#[derive(Deserialize)]
struct ExpectedTables {
    tables: Vec<String>,
}

async fn ensure_tables_exist(pool: &PgPool) {
    let yaml = std::fs::read_to_string(concat!(env!("OUT_DIR"), "/expected_tables.yaml"))
        .expect("Failed to read expected_tables.yaml");
    let config: ExpectedTables =
        serde_yaml::from_str(&yaml).expect("Failed to parse expected_tables.yaml");
    let expected_tables: Vec<&str> = config.tables.iter().map(|s| s.as_str()).collect();

    let existing = sqlx::query_scalar::<_, String>(
        r#"SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE' ORDER BY table_name"#
    ).fetch_all(pool).await
        .expect("Failed to query tables");
    let existing_set: std::collections::HashSet<&str> =
        existing.iter().map(|s| s.as_str()).collect();

    let missing: Vec<&str> = expected_tables
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
