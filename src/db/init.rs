use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::{PgPool, Row};
use tracing::{error, info, warn};

pub enum PartitionGranularity {
    Daily,
    Monthly,
}

pub struct PartitionConfig {
    pub table_name: &'static str,
    pub partition_key: &'static str,
    pub granularity: PartitionGranularity,
}

pub const PARTITION_CONFIGS: &[PartitionConfig] = &[
    PartitionConfig {
        table_name: "tickers",
        partition_key: "trade_date",
        granularity: PartitionGranularity::Daily,
    },
    PartitionConfig {
        table_name: "trades",
        partition_key: "trade_date_utc",
        granularity: PartitionGranularity::Daily,
    },
    PartitionConfig {
        table_name: "candles_seconds",
        partition_key: "candle_date_time_utc",
        granularity: PartitionGranularity::Daily,
    },
    PartitionConfig {
        table_name: "candles_minutes",
        partition_key: "candle_date_time_utc",
        granularity: PartitionGranularity::Monthly,
    },
    PartitionConfig {
        table_name: "candles_days",
        partition_key: "candle_date_time_utc",
        granularity: PartitionGranularity::Monthly,
    },
];

pub async fn init_database(pool: &PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ensure_tables_exist(pool).await?;
    for config in PARTITION_CONFIGS {
        if let Err(e) = fill_partition_gaps(pool, config).await {
            error!(table = config.table_name, error = %e, "Failed to fill partition gaps");
        }
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
    let sql = std::fs::read_to_string("migrations/001_initial.sql")?;
    for statement in sql.split(';') {
        let stmt = statement.trim();
        if !stmt.is_empty() {
            sqlx::query(stmt).execute(pool).await?;
        }
    }
    info!("Executed migrations, created tables");
    Ok(())
}

async fn fill_partition_gaps(pool: &PgPool, config: &PartitionConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let last_partition = get_last_partition(pool, config).await?;

    if let Some(last) = last_partition {
        let today = chrono::Utc::now().naive_utc();
        let partitions_to_create = generate_partitions(config, &last, &today)?;

        for sql in partitions_to_create {
            if let Err(e) = sqlx::query(&sql).execute(pool).await {
                // 이미 존재하면 무시
                warn!(table = config.table_name, sql = %sql, error = %e, "Failed to create partition (may already exist)");
            } else {
                info!(table = config.table_name, partition = %sql, "Created partition");
            }
        }
    } else {
        // 마지막 파티션이 없으면 현재 기간 파티션 생성
        let today = chrono::Utc::now().naive_utc();
        let partitions_to_create = generate_partitions_for_current(config, &today)?;
        for sql in partitions_to_create {
            if let Err(e) = sqlx::query(&sql).execute(pool).await {
                warn!(table = config.table_name, sql = %sql, error = %e, "Failed to create partition");
            }
        }
    }

    Ok(())
}

async fn get_last_partition(pool: &PgPool, config: &PartitionConfig) -> Result<Option<String>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT c.relname AS partition_name
        FROM pg_inherits i
        JOIN pg_class c ON c.oid = i.inhrelid
        JOIN pg_class p ON p.oid = i.inhparent
        WHERE p.relname = $1
        ORDER BY c.relname DESC
        LIMIT 1
        "#,
    ).bind(config.table_name)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r: sqlx::postgres::PgRow| r.get("partition_name")))
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

fn generate_daily_partitions(table_name: &str, last_date: &NaiveDate, today: &NaiveDate) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = *last_date;

    while current < *today {
        let next = current + Days::new(1);
        let partition_name = format_daily_partition_name(table_name, &current);
        let start = NaiveDateTime::new(current, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let end = NaiveDateTime::new(next, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        result.push(build_daily_partition_sql(table_name, &partition_name, &start, &end));
        current = next;
    }
    result
}

fn generate_monthly_partitions(table_name: &str, last_month_start: NaiveDateTime, current_month_start: NaiveDateTime) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = last_month_start;

    while current < current_month_start {
        let next_month = current + Months::new(1);
        let partition_name = format_monthly_partition_name(
            table_name,
            current.year(),
            current.month() as u32,
        );
        result.push(build_monthly_partition_sql(table_name, &partition_name, &current, &next_month));
        current = next_month;
    }
    result
}

fn generate_partitions(config: &PartitionConfig, last_partition: &str, today: &NaiveDateTime) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    match config.granularity {
        PartitionGranularity::Daily => {
            // 일 단위: 파티션명에서 날짜 추출 (예: tickers_y2026m01d01 → 2026-01-01)
            let date_str = extract_daily_partition_date(last_partition)?;
            let last_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?;
            let today_date = today.date();
            Ok(generate_daily_partitions(config.table_name, &last_date, &today_date))
        }
        PartitionGranularity::Monthly => {
            // 월 단위: 파티션명에서 월 추출 (예: candles_minutes_y2026m01 → 2026-01-01T00:00:00)
            let last_month_start = extract_monthly_partition_start(last_partition)?;
            let current_month_start = NaiveDate::from_yo_opt(today.year(), today.ordinal())
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            Ok(generate_monthly_partitions(config.table_name, last_month_start, current_month_start))
        }
    }
}

fn generate_partitions_for_current(config: &PartitionConfig, today: &NaiveDateTime) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    match config.granularity {
        PartitionGranularity::Daily => {
            let today_date = today.date();
            let last_date = today_date;
            Ok(generate_daily_partitions(config.table_name, &last_date, &today_date))
        }
        PartitionGranularity::Monthly => {
            let current_month_start = NaiveDate::from_yo_opt(today.year(), today.ordinal())
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            Ok(generate_monthly_partitions(config.table_name, current_month_start, current_month_start))
        }
    }
}

fn extract_daily_partition_date(partition_name: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let date_part = partition_name
        .trim_start_matches(|c: char| !c.is_ascii_digit() && c != 'y' && c != 'm' && c != 'd');
    if date_part.len() < 9 {
        return Err(format!("Invalid partition name format: {}", partition_name).into());
    }
    let year = &date_part[1..5];
    let month = &date_part[5..7];
    let day = &date_part[7..9];
    Ok(format!("{}-{}-{}", year, month, day))
}

fn extract_monthly_partition_start(partition_name: &str) -> Result<NaiveDateTime, Box<dyn std::error::Error + Send + Sync>> {
    let date_part = partition_name
        .trim_start_matches(|c: char| !c.is_ascii_digit() && c != 'y' && c != 'm');
    if date_part.len() < 7 {
        return Err(format!("Invalid partition name format: {}", partition_name).into());
    }
    let year = date_part[1..5].parse::<i32>()?;
    let month = date_part[5..7].parse::<u32>()?;
    let day = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or("Invalid date")?;
    Ok(day.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()))
}
