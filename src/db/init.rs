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

async fn ensure_tables_exist(pool: &PgPool) {
    let migration_sql = std::fs::read_to_string("migrations/001_initial.sql")
        .expect("Failed to read migration file");

    let drop_tables: std::collections::HashSet<&str> = migration_sql
        .lines()
        .filter(|l| l.trim().to_lowercase().starts_with("drop"))
        .flat_map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() >= 3 && parts[1].to_lowercase() == "table" {
                let name = parts[2].trim_end_matches(';').trim();
                Some(name)
            } else {
                None
            }
        })
        .collect();

    let mut expected_tables: Vec<String> = migration_sql
        .lines()
        .filter(|l| {
            let trimmed = l.trim();
            trimmed.to_lowercase().starts_with("create table ")
                && !trimmed.to_lowercase().contains("partition of")
        })
        .filter_map(|l| {
            let parts: Vec<&str> = l.trim().split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[1].trim_end_matches('(').trim_end_matches(';').trim();
                if !name.is_empty() && !drop_tables.contains(name) {
                    Some(name.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    expected_tables.sort();

    let existing = sqlx::query_scalar::<_, String>(
        r#"SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE' ORDER BY table_name"#
    ).fetch_all(pool).await
        .expect("Failed to query tables");
    let existing_set: std::collections::HashSet<&str> =
        existing.iter().map(|s| s.as_str()).collect();

    let missing: Vec<&str> = expected_tables
        .iter()
        .filter(|t| !existing_set.contains(t.as_str()))
        .map(|t| t.as_str())
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
