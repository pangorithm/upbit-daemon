use chrono::{Datelike, Days, Months, NaiveDate, NaiveTime};
use sqlx::{PgPool, Row};
use tracing::{info, warn};
use crate::config::Config;

const PARTITION_START_TIME: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

/// Partition key format per table — determines FROM/TO bound values
enum PartitionKeyFormat {
    /// VARCHAR(8) → 'YYYYMMDD'
    Tickers,
    /// VARCHAR(10) → 'YYYY-MM-DD'
    Trades,
    /// VARCHAR(20) → 'YYYY-MM-DDTHH:MM:SS'
    Candles,
}

pub async fn ensure_partitions(pool: &PgPool, config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for table in &["tickers", "trades", "candles_seconds"] {
        let key_format = match *table {
            "tickers" => PartitionKeyFormat::Tickers,
            "trades" => PartitionKeyFormat::Trades,
            _ => PartitionKeyFormat::Candles,
        };
        create_daily_partitions(pool, table, config.partition.create, &key_format).await;
    }
    for table in &["candles_minutes", "candles_days"] {
        let key_format = PartitionKeyFormat::Candles;
        create_monthly_partitions(pool, table, config.partition.create, &key_format).await;
    }

    info!("Partitions ensured");
    Ok(())
}

async fn create_daily_partitions(pool: &PgPool, table: &str, n: u32, key_format: &PartitionKeyFormat) {
    let last_date = get_last_partition_date(pool, table).await;

    let start = match last_date {
        Some(d) => d.checked_add_days(Days::new(1)).unwrap(),
        None => chrono::Utc::now().naive_utc().date(),
    };

    let today = chrono::Utc::now().naive_utc().date();
    let start = std::cmp::min(start, today);
    let gap_days = (today - start).num_days() as u32;

    let mut count = 0u32;
    let mut current = start;
    while count < gap_days + n {
        let next = match current.checked_add_days(Days::new(1)) {
            Some(n) => n,
            None => break,
        };
        let (from_str, to_str) = key_format.daily_bounds(&current, &next);

        let name = format!("{}_y{:04}m{:02}d{:02}", table, current.year(), current.month(), current.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            name, table, from_str, to_str
        );
        create_partition(pool, table, &name, &sql).await;
        count += 1;
        current = next;
    }
}

async fn create_monthly_partitions(pool: &PgPool, table: &str, n: u32, key_format: &PartitionKeyFormat) {
    let last_date = get_last_partition_month(pool, table).await;

    let now = chrono::Utc::now().naive_utc().date();

    let current_month = match last_date {
        Some(d) => {
            let next = d + Months::new(1);
            NaiveDate::from_ymd_opt(next.year(), next.month(), 1).unwrap()
        }
        None => NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap(),
    };
    let today_month = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();
    let current_month = std::cmp::min(current_month, today_month);
    let gap_months = months_between(current_month, today_month);

    let mut count = 0u32;
    let mut month_idx = 0u32;
    while count < gap_months + n {
        let future = match current_month + Months::new(month_idx) {
            d if d <= today_month || count < gap_months + n => d,
            _ => break,
        };
        let next_month = future + Months::new(1);
        let (from_str, to_str) = key_format.monthly_bounds(&future, &next_month);

        let name = format!("{}_y{:04}m{:02}", table, future.year(), future.month());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            name, table, from_str, to_str
        );
        create_partition(pool, table, &name, &sql).await;
        count += 1;
        month_idx += 1;
    }
}

impl PartitionKeyFormat {
    fn daily_bounds(&self, from: &NaiveDate, to: &NaiveDate) -> (String, String) {
        match self {
            PartitionKeyFormat::Tickers => (
                format!("{:04}{:02}{:02}", from.year(), from.month(), from.day()),
                format!("{:04}{:02}{:02}", to.year(), to.month(), to.day()),
            ),
            PartitionKeyFormat::Trades => (
                format!("{:04}-{:02}-{:02}", from.year(), from.month(), from.day()),
                format!("{:04}-{:02}-{:02}", to.year(), to.month(), to.day()),
            ),
            PartitionKeyFormat::Candles => (
                from.and_time(PARTITION_START_TIME).format("%Y-%m-%dT%H:%M:%S").to_string(),
                to.and_time(PARTITION_START_TIME).format("%Y-%m-%dT%H:%M:%S").to_string(),
            ),
        }
    }

    fn monthly_bounds(&self, from: &NaiveDate, to: &NaiveDate) -> (String, String) {
        let from_dt = from.and_time(PARTITION_START_TIME);
        let to_dt = to.and_time(PARTITION_START_TIME);
        (
            from_dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
            to_dt.format("%Y-%m-%dT%H:%M:%S").to_string(),
        )
    }
}

fn months_between(from: NaiveDate, to: NaiveDate) -> u32 {
    ((to.year() * 12 + to.month() as i32) - (from.year() * 12 + from.month() as i32)) as u32
}

async fn get_last_partition_date(pool: &PgPool, table: &str) -> Option<NaiveDate> {
    let rows = sqlx::query(
        r#"
        SELECT c.relname AS partition_name
        FROM pg_inherits i
        JOIN pg_class c ON c.oid = i.inhrelid
        JOIN pg_class p ON p.oid = i.inhparent
        WHERE p.relname = $1
        ORDER BY c.relname DESC
        LIMIT 1
        "#,
    )
    .bind(table)
    .fetch_all(pool)
    .await;

    match rows {
        Ok(rows) if !rows.is_empty() => {
            let name: String = rows[0].get("partition_name");
            extract_date_from_partition_name(&name)
        }
        _ => None,
    }
}

async fn get_last_partition_month(pool: &PgPool, table: &str) -> Option<NaiveDate> {
    let rows = sqlx::query(
        r#"
        SELECT c.relname AS partition_name
        FROM pg_inherits i
        JOIN pg_class c ON c.oid = i.inhrelid
        JOIN pg_class p ON p.oid = i.inhparent
        WHERE p.relname = $1
        ORDER BY c.relname DESC
        LIMIT 1
        "#,
    )
    .bind(table)
    .fetch_all(pool)
    .await;

    match rows {
        Ok(rows) if !rows.is_empty() => {
            let name: String = rows[0].get("partition_name");
            extract_month_from_partition_name(&name)
        }
        _ => None,
    }
}

fn extract_date_from_partition_name(name: &str) -> Option<NaiveDate> {
    let suffix = name.rsplit('_').next()?;
    if !suffix.starts_with('y') {
        return None;
    }
    let without_y = &suffix[1..];
    let m_pos = without_y.find('m')?;
    let d_pos = without_y[m_pos + 1..].find('d')?;
    let year = without_y[..m_pos].parse().ok()?;
    let month = without_y[m_pos + 1..m_pos + 1 + d_pos].parse().ok()?;
    let day = without_y[m_pos + 1 + d_pos + 1..].parse().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

fn extract_month_from_partition_name(name: &str) -> Option<NaiveDate> {
    let suffix = name.rsplit('_').next()?;
    if !suffix.starts_with('y') {
        return None;
    }
    let without_y = &suffix[1..];
    let m_pos = without_y.find('m')?;
    let year = without_y[..m_pos].parse().ok()?;
    let month = without_y[m_pos + 1..].parse().ok()?;
    NaiveDate::from_ymd_opt(year, month, 1)
}

async fn create_partition(pool: &PgPool, table_name: &str, partition_name: &str, sql: &str) {
    let exists = sqlx::query(
        r#"SELECT 1 FROM pg_class WHERE relname = $1"#,
    )
    .bind(partition_name)
    .fetch_one(pool)
    .await;

    if exists.is_ok() {
        info!(table = table_name, partition = partition_name, "Skipping partition (already exists)");
        return;
    }

    match sqlx::query(sql).execute(pool).await {
        Ok(_) => info!("Created partition for {} ({})", table_name, partition_name),
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("would overlap") {
                info!(table = table_name, partition = partition_name, "Skipping partition (would overlap)");
            } else {
                warn!(table = table_name, error = %e, "Failed to create partition");
            }
        }
    }
}
