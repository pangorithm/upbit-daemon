use chrono::{Datelike, Days, Months, NaiveDate};
use sqlx::{PgPool, Row};
use tracing::{debug, info, warn};
use crate::config::Config;

struct ExistingPartition {
    from_val: String,
    to_val: String,
}

pub async fn create_future_partitions(pool: &PgPool, config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let today = chrono::Utc::now().naive_utc();
    let today_date = today.date();
    let n = config.partition.create;

    // 일별 파티션
    let existing = get_existing_partition_ranges(pool, "tickers").await;
    create_daily_string_partitions(pool, "tickers", &today_date, n, &existing).await;

    let existing = get_existing_partition_ranges(pool, "trades").await;
    create_daily_iso_partitions(pool, "trades", &today_date, n, &existing).await;

    let existing = get_existing_partition_ranges(pool, "candles_seconds").await;
    create_daily_ts_partitions(pool, "candles_seconds", &today_date, n, &existing).await;

    // 월별 파티션
    let existing = get_existing_partition_ranges(pool, "candles_minutes").await;
    create_monthly_partitions(pool, "candles_minutes", &today_date, n, &existing).await;

    let existing = get_existing_partition_ranges(pool, "candles_days").await;
    create_monthly_partitions(pool, "candles_days", &today_date, n, &existing).await;

    info!("Future partitions created");
    Ok(())
}

async fn get_existing_partition_ranges(pool: &PgPool, table_name: &str) -> Vec<ExistingPartition> {
    let rows = match sqlx::query(
        r#"
        SELECT c.relname,
               pg_get_expr(c.relpartbound, c.oid) AS bound
        FROM pg_inherits i
        JOIN pg_class c ON c.oid = i.inhrelid
        JOIN pg_class p ON p.oid = i.inhparent
        WHERE p.relname = $1
        ORDER BY c.relname
        "#,
    )
    .bind(table_name)
    .fetch_all(pool)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            debug!(table = table_name, error = %e, "Failed to query existing partition ranges");
            return Vec::new();
        }
    };

    let existing: Vec<_> = rows.iter()
        .filter_map(|row| {
            let bound: String = row.get("bound");
            let from = extract_bound_from(&bound);
            let to = extract_bound_to(&bound);
             match (from, to) {
                (Some(f), Some(t)) => Some(ExistingPartition {
                    from_val: f,
                    to_val: t,
                }),
                _ => None,
            }
        })
        .collect();
    existing
}

fn extract_bound_from(bound: &str) -> Option<String> {
    let from_pos = bound.find("FROM (")?;
    let rest = &bound[from_pos + 5..];
    let end = rest.find(')')?;
    Some(rest[..end].trim().to_string())
}

fn extract_bound_to(bound: &str) -> Option<String> {
    let to_pos = bound.find("TO (")?;
    let rest = &bound[to_pos + 4..];
    let end = rest.find(')')?;
    Some(rest[..end].trim().to_string())
}

fn normalize_ts(s: &str) -> String {
    s.replace('T', " ")
}

fn overlaps(new_from: &str, new_to: &str, existing: &[ExistingPartition]) -> bool {
    let nf = normalize_ts(new_from);
    let nt = normalize_ts(new_to);
    existing.iter().any(|ep| {
        nf < ep.to_val && ep.from_val < nt
    })
}

async fn create_daily_string_partitions(pool: &PgPool, table: &str, today: &NaiveDate, n: u32, existing: &[ExistingPartition]) {
    for i in 0..n {
        let d = today.checked_add_days(Days::new(i as u64)).unwrap();
        let next = d + Days::new(1);
        let from_str = format!("{:04}{:02}{:02}", d.year(), d.month(), d.day());
        let to_str = format!("{:04}{:02}{:02}", next.year(), next.month(), next.day());

        if overlaps(&from_str, &to_str, existing) {
            continue;
        }

        let name = format!("{}_y{:04}m{:02}d{:02}", table, d.year(), d.month(), d.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            name, table, from_str, to_str
        );
        create_partition(pool, table, &sql).await;
    }
}

async fn create_daily_ts_partitions(pool: &PgPool, table: &str, today: &NaiveDate, n: u32, existing: &[ExistingPartition]) {
    for i in 0..n {
        let d = today.checked_add_days(Days::new(i as u64)).unwrap();
        let end = d + Days::new(1);
        let from_str = d.and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()).to_string();
        let to_str = end.and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()).to_string();

        if overlaps(&from_str, &to_str, existing) {
            continue;
        }

        let name = format!("{}_y{:04}m{:02}d{:02}", table, d.year(), d.month(), d.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            name, table, from_str, to_str
        );
        create_partition(pool, table, &sql).await;
    }
}

async fn create_daily_iso_partitions(pool: &PgPool, table: &str, today: &NaiveDate, n: u32, existing: &[ExistingPartition]) {
    for i in 0..n {
        let d = today.checked_add_days(Days::new(i as u64)).unwrap();
        let next = d + Days::new(1);
        let from_str = d.format("%Y-%m-%d").to_string();
        let to_str = next.format("%Y-%m-%d").to_string();

        if overlaps(&from_str, &to_str, existing) {
            continue;
        }

        let name = format!("{}_y{:04}m{:02}d{:02}", table, d.year(), d.month(), d.day());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            name, table, from_str, to_str
        );
        create_partition(pool, table, &sql).await;
    }
}

async fn create_monthly_partitions(pool: &PgPool, table: &str, today: &NaiveDate, n: u32, existing: &[ExistingPartition]) {
    for i in 0..n {
        let future = NaiveDate::from_ymd_opt(today.year(), today.month() + i as u32, 1).unwrap();
        let next_month = future + Months::new(1);
        let from_dt = future.and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let to_dt = next_month.and_time(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let from_str = from_dt.format("%Y-%m-%dT%H:%M:%S").to_string();
        let to_str = to_dt.format("%Y-%m-%dT%H:%M:%S").to_string();

        if overlaps(&from_str, &to_str, existing) {
            continue;
        }

        let name = format!("{}_y{:04}m{:02}", table, future.year(), future.month());
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} PARTITION OF {} FOR VALUES FROM ('{}') TO ('{}')",
            name, table, from_str, to_str
        );
        create_partition(pool, table, &sql).await;
    }
}

async fn create_partition(pool: &PgPool, table_name: &str, sql: &str) {
    if let Err(e) = sqlx::query(sql).execute(pool).await {
        let err_str = e.to_string();
        if err_str.contains("would overlap") {
            info!(table = table_name, sql = %sql, "Skipping partition (already exists with overlapping range)");
        } else {
            warn!(table = table_name, error = %e, "Failed to create future partition");
        }
    } else {
        let partition_name = sql.split("CREATE TABLE IF NOT EXISTS ")
            .nth(1)
            .and_then(|s| s.split(" PARTITION OF ").next())
            .unwrap_or("unknown");
        info!("Created future partition for {} ({})", table_name, partition_name);
    }
}
