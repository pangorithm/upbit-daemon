use chrono::{Days, Months, NaiveDateTime};
use sqlx::{PgPool, Row};
use tracing::{error, info};
use crate::config::ApiConfig;

pub async fn delete_daily_partitions(pool: &PgPool, config: &ApiConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let daily_tables = ["tickers", "trades", "candles_seconds"];
    let cutoff_date = chrono::Utc::now().naive_utc().date() - Days::new(config.partition_retain_days as u64);
    let cutoff_str = cutoff_date.format("%Y-%m-%dT%H:%M:%S").to_string();

    for table_name in &daily_tables {
        let partitions_to_delete = get_partitions_to_delete(pool, table_name, &cutoff_str).await?;
        for (partition_name, sql) in partitions_to_delete {
            if let Err(e) = sqlx::query(&sql).execute(pool).await {
                error!(table = table_name, partition = %partition_name, error = %e, "Failed to delete partition");
            } else {
                info!(table = table_name, partition = %partition_name, "Deleted old partition");
            }
        }
    }

    Ok(())
}

pub async fn delete_monthly_partitions(pool: &PgPool, config: &ApiConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let monthly_tables = ["candles_minutes", "candles_days"];
    let cutoff = chrono::Utc::now().naive_utc() - Months::new(config.partition_retain_months as u32);
    let cutoff_str = cutoff.format("%Y-%m-%dT%H:%M:%S").to_string();

    for table_name in &monthly_tables {
        let partitions_to_delete = get_partitions_to_delete(pool, table_name, &cutoff_str).await?;
        for (partition_name, sql) in partitions_to_delete {
            if let Err(e) = sqlx::query(&sql).execute(pool).await {
                error!(table = table_name, partition = %partition_name, error = %e, "Failed to delete partition");
            } else {
                info!(table = table_name, partition = %partition_name, "Deleted old partition");
            }
        }
    }

    Ok(())
}

async fn get_partitions_to_delete(pool: &PgPool, table_name: &str, cutoff: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = sqlx::query(
        r#"SELECT partition_name, partition_bounds FROM information_schema.partitions WHERE table_name = $1 AND partition_bounds < $2 ORDER BY partition_bounds ASC"#,
    ).bind(table_name).bind(cutoff)
    .fetch_all(pool).await?;

    let mut result = Vec::new();
    for row in rows {
        let partition_name: String = row.get("partition_name");
        let partition_bounds: String = row.get("partition_bounds");
        let start_str = extract_partition_start(&partition_bounds);
        let _start_dt = NaiveDateTime::parse_from_str(&start_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| format!("Failed to parse partition start '{}': {}", start_str, e))?;

        let sql = format!("DROP TABLE IF EXISTS {}", partition_name);
        result.push((partition_name, sql));
    }

    Ok(result)
}

fn extract_partition_start(bounds: &str) -> String {
    bounds.trim_start_matches("('")
        .split(", '")
        .next()
        .unwrap_or(bounds)
        .to_string()
}
