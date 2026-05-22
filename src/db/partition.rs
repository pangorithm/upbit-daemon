use chrono::{Datelike, Days, Months, NaiveDate};
use sqlx::PgPool;
use tracing::{info, warn};
use crate::config::Config;

pub async fn create_future_partitions(pool: &PgPool, config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let today = chrono::Utc::now().naive_utc();
    let today_date = today.date();
    let n = config.partition.create;

    // 일별 파티션: 각 테이블이 동일한 N 일.future 날짜를 생성
    // tickers, trades: VARCHAR(8) 파티션 키 → 'YYYYMMDD' 포맷
    create_daily_string_partitions(pool, "tickers", &today_date, n).await;
    create_daily_string_partitions(pool, "trades", &today_date, n).await;

    // candles_seconds: TIMESTAMP WITH TIME ZONE 파티션 키 → ISO 형식
    create_daily_ts_partitions(pool, "candles_seconds", &today_date, n).await;

    // 월별 파티션
    for i in 0..n {
        let future = NaiveDate::from_ymd_opt(today_date.year(), today_date.month() + i as u32, 1).unwrap();
        let next_month = future + Months::new(1);

        let name = format!("candles_days_y{:04}m{:02}", future.year(), future.month());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF candles_days FOR VALUES FROM ('{}') TO ('{}')",
            name, future.format("%Y-%m-%d"), next_month.format("%Y-%m-%d")
        );
        create_partition(pool, "candles_days", &sql).await;

        let name = format!("candles_minutes_y{:04}m{:02}", future.year(), future.month());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF candles_minutes FOR VALUES FROM ('{}') TO ('{}')",
            name,
            future.format("%Y-%m-%dT%H:%M:%S"),
            next_month.format("%Y-%m-%dT%H:%M:%S")
        );
        create_partition(pool, "candles_minutes", &sql).await;
    }

    info!("Future partitions created");
    Ok(())
}

async fn create_daily_string_partitions(pool: &PgPool, table: &str, today: &NaiveDate, n: u32) {
    for i in 0..n {
        let d = today.checked_add_days(Days::new(i as u64)).unwrap();
        let next = d + Days::new(1);
        let name = format!("{}_y{:04}m{:02}d{:02}", table, d.year(), d.month(), d.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{:04}{:02}{:02}') TO ('{:04}{:02}{:02}')",
            name, table, d.year(), d.month(), d.day(),
            next.year(), next.month(), next.day()
        );
        create_partition(pool, table, &sql).await;
    }
}

async fn create_daily_ts_partitions(pool: &PgPool, table: &str, today: &NaiveDate, n: u32) {
    for i in 0..n {
        let d = today.checked_add_days(Days::new(i as u64)).unwrap();
        let end = d + Days::new(1);
        let name = format!("{}_y{:04}m{:02}d{:02}", table, d.year(), d.month(), d.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            name, table,
            d.format("%Y-%m-%dT%H:%M:%S"), end.format("%Y-%m-%dT%H:%M:%S")
        );
        create_partition(pool, table, &sql).await;
    }
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
