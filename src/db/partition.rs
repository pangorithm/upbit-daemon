use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::PgPool;
use tracing::{info, warn};
use crate::config::ApiConfig;

pub async fn create_future_partitions(pool: &PgPool, config: &ApiConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let daily_tables = [
        ("tickers", "trade_date"),
        ("trades", "trade_date_utc"),
        ("candles_seconds", "candle_date_time_utc"),
    ];
    let monthly_tables = [
        ("candles_minutes", "candle_date_time_utc"),
        ("candles_days", "candle_date_time_utc"),
    ];

    let today = chrono::Utc::now().naive_utc();
    let today_date = today.date();

    for (table_name, _) in &daily_tables {
        let partitions = generate_future_daily_partitions(table_name, &today_date, config.partition_create as usize);
        for sql in partitions {
            create_partition(pool, table_name, &sql).await;
        }
    }

    for (table_name, _) in &monthly_tables {
        let partitions = generate_future_monthly_partitions(table_name, config.partition_create as usize);
        for sql in partitions {
            create_partition(pool, table_name, &sql).await;
        }
    }

    info!("Future partitions created");
    Ok(())
}

fn generate_future_daily_partitions(table_name: &str, from_date: &NaiveDate, count: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = *from_date;

    for _ in 0..count {
        let next = current + Days::new(1);
        let partition_name = format_daily_partition_name(table_name, &current);
        let start = NaiveDateTime::new(current, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let end = NaiveDateTime::new(next, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        result.push(build_daily_partition_sql(table_name, &partition_name, &start, &end));
        current = next;
    }
    result
}

fn generate_future_monthly_partitions(table_name: &str, count: usize) -> Vec<String> {
    let mut result = Vec::new();
    let now = chrono::Utc::now().naive_utc();
    let current_year = now.year();
    let current_month = now.month();

    for i in 0..count {
        let future = NaiveDate::from_ymd_opt(current_year, current_month + i as u32, 1)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let next_month = future + Months::new(1);
        let partition_name = format_monthly_partition_name(table_name, current_year, current_month + i as u32);
        result.push(build_monthly_partition_sql(table_name, &partition_name, &future, &next_month));
    }
    result
}

fn format_daily_partition_name(table_name: &str, date: &NaiveDate) -> String {
    format!("{}_y{:04}m{:02}d{:02}", table_name, date.year(), date.month(), date.day())
}

fn format_monthly_partition_name(table_name: &str, year: i32, month: u32) -> String {
    format!("{}_y{:04}m{:02}", table_name, year, month)
}

fn build_daily_partition_sql(table_name: &str, partition_name: &str, start: &NaiveDateTime, end: &NaiveDateTime) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
        partition_name, table_name, start.format("%Y-%m-%dT%H:%M:%S"), end.format("%Y-%m-%dT%H:%M:%S")
    )
}

fn build_monthly_partition_sql(table_name: &str, partition_name: &str, start: &NaiveDateTime, end: &NaiveDateTime) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
        partition_name, table_name, start.format("%Y-%m-%dT%H:%M:%S"), end.format("%Y-%m-%dT%H:%M:%S")
    )
}

async fn create_partition(pool: &PgPool, table_name: &str, sql: &str) {
    if let Err(e) = sqlx::query(sql).execute(pool).await {
        warn!(table = table_name, sql = %sql, error = %e, "Failed to create future partition");
    } else {
        let partition_name = sql.split("CREATE TABLE IF NOT EXISTS ").nth(1)
            .and_then(|s| s.split(" PARTITION OF ").next())
            .unwrap_or("?");
        info!(table = table_name, partition = partition_name, "Created future partition");
    }
}
