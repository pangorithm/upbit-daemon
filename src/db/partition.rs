use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::PgPool;
use tracing::{info, warn};
use crate::config::ApiConfig;

pub async fn create_future_partitions(pool: &PgPool, config: &ApiConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let today = chrono::Utc::now().naive_utc();
    let today_date = today.date();

    for _ in 0..config.partition_create {
        let next = today_date + Days::new(1);
        let partition_name = format!("tickers_y{:04}m{:02}d{:02}", today_date.year(), today_date.month(), today_date.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF tickers FOR VALUES FROM ('{:04}{:02}{:02}') TO ('{:04}{:02}{:02}')",
            partition_name, today_date.year(), today_date.month(), today_date.day(),
            next.year(), next.month(), next.day()
        );
        create_partition(pool, "tickers", &sql).await;
        today_date.checked_add_days(Days::new(1));
    }

    for _ in 0..config.partition_create {
        let next = today_date + Days::new(1);
        let partition_name = format!("trades_y{:04}m{:02}d{:02}", today_date.year(), today_date.month(), today_date.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF trades FOR VALUES FROM ('{}') TO ('{}')",
            partition_name,
            today_date.format("%Y-%m-%d"),
            next.format("%Y-%m-%d")
        );
        create_partition(pool, "trades", &sql).await;
        today_date.checked_add_days(Days::new(1));
    }

    for _ in 0..config.partition_create {
        let end = today_date + Days::new(1);
        let partition_name = format!("candles_seconds_y{:04}m{:02}d{:02}", today_date.year(), today_date.month(), today_date.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF candles_seconds FOR VALUES FROM ('{}') TO ('{})",
            partition_name,
            today_date.format("%Y-%m-%dT%H:%M:%S"),
            end.format("%Y-%m-%dT%H:%M:%S")
        );
        create_partition(pool, "candles_seconds", &sql).await;
    }

   let today_date = today.date();
    for i in 0..config.partition_create {
        let future = NaiveDate::from_ymd_opt(today_date.year(), today_date.month() + i as u32, 1).unwrap();
        let next_month = future + Months::new(1);
        let partition_name = format!("candles_days_y{:04}m{:02}", future.year(), future.month());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF candles_days FOR VALUES FROM ('{}') TO ('{}')",
            partition_name,
            future.format("%Y-%m-%d"),
            next_month.format("%Y-%m-%d")
        );
        create_partition(pool, "candles_days", &sql).await;
    }

    for i in 0..config.partition_create {
        let future = NaiveDate::from_ymd_opt(today_date.year(), today_date.month() + i as u32, 1).unwrap();
        let next_month = future + Months::new(1);
        let partition_name = format!("candles_minutes_y{:04}m{:02}", future.year(), future.month());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF candles_minutes FOR VALUES FROM ('{}') TO ('{}')",
            partition_name,
            future.format("%Y-%m-%dT%H:%M:%S"),
            next_month.format("%Y-%m-%dT%H:%M:%S")
        );
        create_partition(pool, "candles_minutes", &sql).await;
    }

    for i in 0..config.partition_create {
        let future = NaiveDate::from_ymd_opt(today.year(), today.month() + i as u32, 1).unwrap();
        let next_month = future + Months::new(1);
        let partition_name = format!("candles_minutes_y{:04}m{:02}", future.year(), future.month());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF candles_minutes FOR VALUES FROM ('{}') TO ('{})",
            partition_name,
            future.format("%Y-%m-%dT%H:%M:%S"),
            next_month.format("%Y-%m-%dT%H:%M:%S")
        );
        create_partition(pool, "candles_minutes", &sql).await;
    }

    info!("Future partitions created");
    Ok(())
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
