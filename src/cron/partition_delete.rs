use chrono::{Days, Months};
use sqlx::{PgPool, Row};
use tracing::{error, info};
use crate::config::Config;

pub async fn delete_daily_partitions(pool: &PgPool, config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let daily_tables = ["tickers", "trades", "candles_seconds"];
    let cutoff_date = chrono::Utc::now().naive_utc().date() - Days::new(config.partition.retain_days as u64);

    for table_name in &daily_tables {
        let partitions_to_delete = get_partitions_to_delete(pool, table_name, &cutoff_date).await?;
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

pub async fn delete_monthly_partitions(pool: &PgPool, config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let monthly_tables = ["candles_minutes", "candles_days"];
    let cutoff = chrono::Utc::now().naive_utc() - Months::new(config.partition.retain_months as u32);
    let cutoff_date = cutoff.date();

    for table_name in &monthly_tables {
        let partitions_to_delete = get_partitions_to_delete(pool, table_name, &cutoff_date).await?;
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

async fn get_partitions_to_delete(pool: &PgPool, table_name: &str, cutoff_date: &chrono::NaiveDate) -> Result<Vec<(String, String)>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = sqlx::query(
        r#"
        SELECT c.relname AS partition_name, pg_get_expr(c.relpartbound, c.oid) AS bound_expr
        FROM pg_inherits i
        JOIN pg_class c ON c.oid = i.inhrelid
        JOIN pg_class p ON p.oid = i.inhparent
        WHERE p.relname = $1
        ORDER BY c.relname ASC
        "#,
    ).bind(table_name)
    .fetch_all(pool).await?;

    let mut result = Vec::new();
    for row in rows {
        let partition_name: String = row.get("partition_name");
        let _bound_expr: String = row.get("bound_expr");

        let partition_date = extract_partition_date(&partition_name, table_name)?;
        if partition_date < *cutoff_date {
            let sql = format!("DROP TABLE IF EXISTS {}", partition_name);
            result.push((partition_name, sql));
        }
    }

    Ok(result)
}

fn extract_partition_date(partition_name: &str, table_name: &str) -> Result<chrono::NaiveDate, Box<dyn std::error::Error + Send + Sync>> {
    let date_part = partition_name
        .split('_')
        .nth(1)
        .ok_or_else(|| format!("Invalid partition name: {}", partition_name))?;

    if !date_part.starts_with('y') {
        return Err(format!("Unknown date format in partition: {}", partition_name).into());
    }

    let year = date_part[1..5].parse::<i32>()?;
    let month = date_part[5..7].parse::<u32>()?;

    if table_name == "tickers" || table_name == "trades" || table_name == "candles_seconds" {
        let day = date_part[7..9].parse::<u32>()?;
        Ok(chrono::NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")?)
    } else {
        Ok(chrono::NaiveDate::from_ymd_opt(year, month, 1).ok_or("Invalid date")?)
    }
}
