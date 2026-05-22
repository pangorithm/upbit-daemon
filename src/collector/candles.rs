use crate::config::Config;
use crate::api::quotation::candle;
use chrono::Duration;
use sqlx::Row;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::interval;
use tracing::{error, info};

#[derive(Debug, Clone)]
enum ApiRequestDto {
    CandlesMinutes {
        market: String,
        count: u32,
        to: String,
        unit: u32,
    },
}

type ApiQueue = Arc<Mutex<VecDeque<ApiRequestDto>>>;

fn get_global_queue() -> ApiQueue {
    use std::sync::OnceLock;
    static QUEUE: OnceLock<ApiQueue> = OnceLock::new();
    QUEUE
        .get_or_init(|| Arc::new(Mutex::new(VecDeque::new())))
        .clone()
}

fn candle_table_for_unit(unit: u32) -> &'static str {
    if unit == 1 {
        "candles_seconds"
    } else if unit >= 60 && unit % 60 == 0 {
        "candles_days"
    } else {
        "candles_minutes"
    }
}

pub async fn fill_candle_gap(
    pool: &PgPool,
    rest: &crate::api::rest::RestClient,
    config: &Config,
    market: &str,
    unit: u32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let last_candle_time = get_last_candle_time(pool, market, unit).await?;
    let last_candle_time = last_candle_time.ok_or("No last candle found")?;

    let gap_minutes = calculate_gap_minutes(Some(&last_candle_time))?;
    if gap_minutes == 0 {
        info!(market, unit, "No gap in candle data");
        return Ok(());
    }

    info!(market, gap_minutes, unit, "Adding gap-filling to global queue");

    let queue = get_global_queue();
    let total_candles_needed = gap_minutes / unit;
    let mut remaining_candles = total_candles_needed;

    let mut current_from = last_candle_time
        .parse::<chrono::DateTime<chrono::Utc>>()
        .expect("Failed to parse last_candle_time");

    while remaining_candles > 0 {
        let batch_size = std::cmp::min(remaining_candles, config.candle.count);
        let to_str = current_from
            .checked_add_signed(Duration::minutes((batch_size * unit) as i64))
            .expect("Time overflow")
            .format("%Y-%m-%dT%H:%M:%S+00:00")
            .to_string();
        queue.lock().await.push_back(ApiRequestDto::CandlesMinutes {
            market: market.to_string(),
            count: batch_size,
            to: to_str,
            unit,
        });
        current_from = current_from
            .checked_add_signed(Duration::minutes((batch_size * unit) as i64))
            .expect("Time overflow");
        remaining_candles -= batch_size;
    }

    start_background_task(queue, pool.clone(), rest.clone(), config.clone());
    info!(market, unit, "Gap filled successfully");

    Ok(())
}

pub async fn fill_all_candle_gaps(
    pool: &PgPool,
    rest: &crate::api::rest::RestClient,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let markets_rows = sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(pool).await?;
    let markets: Vec<String> = markets_rows.iter()
        .map(|r: &sqlx::postgres::PgRow| r.get("market"))
        .collect();

    for market in &markets {
        for &unit in &config.candle.units {
            if let Err(e) = fill_candle_gap(pool, rest, config, market, unit).await {
                error!(market, unit, error = %e, "Failed to fill candle gap");
            }
        }
    }

    Ok(())
}

fn start_background_task(
    queue: ApiQueue,
    pool: PgPool,
    rest: crate::api::rest::RestClient,
    config: Config,
) -> std::sync::Arc<JoinHandle<()>> {
    use std::sync::{Arc, OnceLock};
    static HANDLE: OnceLock<Arc<JoinHandle<()>>> = OnceLock::new();

    HANDLE.get_or_init(|| {
        Arc::new(tokio::spawn(async move {
            let rest_client = rest.clone();
            let mut timer = interval(std::time::Duration::from_secs_f64(
                1.0 / config.rate_limit.api_calls_per_second as f64,
            ));

            loop {
                timer.tick().await;

                let batch = {
                    let mut q = queue.lock().await;
                    q.pop_front()
                };

                match batch {
                    Some(request) => match request {
                        ApiRequestDto::CandlesMinutes {
                            market,
                            count,
                            to,
                            unit,
                        } => {
                            match candle::get_candles_minutes(
                                &rest_client, &market, unit, count, &to,
                            )
                            .await
                            {
                                Ok(candles) => {
                                    if let Err(e) = insert_candles(&pool, &candles).await {
                                        error!(
                                            api_type = "CandlesMinutes",
                                            market,
                                            error = e.to_string(),
                                            "Failed to insert candles from background task"
                                        );
                                    } else {
                                        info!(
                                            api_type = "CandlesMinutes",
                                            market,
                                            count = candles.len(),
                                            "Batch fetched and inserted via global queue"
                                        );
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        api_type = "CandlesMinutes",
                                        market,
                                        count,
                                        error = e.to_string(),
                                        "Failed to fetch candles"
                                    );
                                }
                            }
                        }
                    },
                    None => {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                }
            }
        }))
    }).clone()
}

async fn get_last_candle_time(pool: &PgPool, market: &str, unit: u32) -> Result<Option<String>, sqlx::Error> {
    let table = candle_table_for_unit(unit);
    let query = match table {
        "candles_seconds" => {
            r#"SELECT candle_date_time_utc FROM candles_seconds WHERE market = $1 ORDER BY candle_date_time_utc DESC LIMIT 1"#
        }
        "candles_days" => {
            r#"SELECT candle_date_time_utc FROM candles_days WHERE market = $1 ORDER BY candle_date_time_utc DESC LIMIT 1"#
        }
        _ => {
            r#"SELECT candle_date_time_utc FROM candles_minutes WHERE market = $1 AND unit = $2 ORDER BY candle_date_time_utc DESC LIMIT 1"#
        }
    };

    if table == "candles_days" || table == "candles_seconds" {
        sqlx::query(query)
            .bind(market)
            .fetch_optional(pool)
            .await
            .map(|row| row.map(|r: sqlx::postgres::PgRow| r.get("candle_date_time_utc")))
    } else {
        sqlx::query(query)
            .bind(market)
            .bind(unit as i32)
            .fetch_optional(pool)
            .await
            .map(|row| row.map(|r: sqlx::postgres::PgRow| r.get("candle_date_time_utc")))
    }
}

fn calculate_gap_minutes(
    last_candle_time: Option<&str>,
) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
    match last_candle_time {
        Some(time_str) => {
            let last_time = chrono::DateTime::parse_from_rfc3339(&time_str)
                .map_err(|e| format!("Failed to parse time '{}': {}", time_str, e))?;
            let now = chrono::Utc::now();
            let diff = now - last_time.with_timezone(&chrono::Utc);
            let minutes = diff.num_minutes();
            if minutes < 0 {
                Ok(0)
            } else {
                Ok(minutes as u32)
            }
        }
        None => Ok(0),
    }
}

async fn insert_candles(
    pool: &PgPool,
    candles: &[Value],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for candle in candles {
        let market = candle["market"].as_str().unwrap_or("");
        let candle_date_time_utc = candle["candle_date_time_utc"].as_str().unwrap_or("");
        let candle_date_time_kst = candle["candle_date_time_kst"].as_str().unwrap_or("");
        let opening_price = candle["opening_price"].as_f64().unwrap_or(0.0);
        let high_price = candle["high_price"].as_f64().unwrap_or(0.0);
        let low_price = candle["low_price"].as_f64().unwrap_or(0.0);
        let trade_price = candle["trade_price"].as_f64().unwrap_or(0.0);
        let candle_acc_trade_price = candle["candle_acc_trade_price"].as_f64().unwrap_or(0.0);
        let candle_acc_trade_volume = candle["candle_acc_trade_volume"].as_f64().unwrap_or(0.0);
        let unit = candle["unit"].as_u64().unwrap_or(0) as u32;
        let table = candle_table_for_unit(unit);

        match table {
            "candles_days" => {
                let prev_closing_price = candle["prev_closing_price"].as_f64().unwrap_or(0.0);
                let change_price = candle["change_price"].as_f64().unwrap_or(0.0);
                let change_rate = candle["change_rate"].as_f64().unwrap_or(0.0);
                let converted_trade_price = candle["converted_trade_price"].as_f64();
                let timestamp = candle["timestamp"].as_i64().unwrap_or(0);
                sqlx::query(
                    r#"
                    INSERT INTO candles_days (
                        market, candle_date_time_utc, candle_date_time_kst,
                        opening_price, high_price, low_price, trade_price,
                        timestamp, candle_acc_trade_price, candle_acc_trade_volume,
                        prev_closing_price, change_price, change_rate, converted_trade_price
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                    ON CONFLICT DO NOTHING
                    "#,
                )
                .bind(market)
                .bind(candle_date_time_utc)
                .bind(candle_date_time_kst)
                .bind(opening_price)
                .bind(high_price)
                .bind(low_price)
                .bind(trade_price)
                .bind(timestamp)
                .bind(candle_acc_trade_price)
                .bind(candle_acc_trade_volume)
                .bind(prev_closing_price)
                .bind(change_price)
                .bind(change_rate)
                .bind(converted_trade_price)
                .execute(pool).await
                .map_err(|e| {
                    error!(table, error = e.to_string(), market, "Failed to insert candle to candles_days");
                    e
                })?;
            }
            "candles_seconds" => {
                let timestamp = candle["timestamp"].as_i64().unwrap_or(0);
                sqlx::query(
                    r#"
                    INSERT INTO candles_seconds (
                        market, candle_date_time_utc, candle_date_time_kst,
                        opening_price, high_price, low_price, trade_price,
                        timestamp, candle_acc_trade_price, candle_acc_trade_volume
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                    ON CONFLICT DO NOTHING
                    "#,
                )
                .bind(market)
                .bind(candle_date_time_utc)
                .bind(candle_date_time_kst)
                .bind(opening_price)
                .bind(high_price)
                .bind(low_price)
                .bind(trade_price)
                .bind(timestamp)
                .bind(candle_acc_trade_price)
                .bind(candle_acc_trade_volume)
                .execute(pool).await
                .map_err(|e| {
                    error!(table, error = e.to_string(), market, "Failed to insert candle to candles_seconds");
                    e
                })?;
            }
            _ => {
                let unit = candle["unit"].as_u64().unwrap_or(0) as i64;
                sqlx::query(
                    r#"
                    INSERT INTO candles_minutes (
                        market, candle_date_time_utc, candle_date_time_kst,
                        opening_price, high_price, low_price, trade_price,
                        candle_acc_trade_price, candle_acc_trade_volume, unit
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                    ON CONFLICT (market, candle_date_time_utc) DO UPDATE SET
                        candle_date_time_kst = EXCLUDED.candle_date_time_kst,
                        opening_price = EXCLUDED.opening_price,
                        high_price = EXCLUDED.high_price,
                        low_price = EXCLUDED.low_price,
                        trade_price = EXCLUDED.trade_price,
                        candle_acc_trade_price = EXCLUDED.candle_acc_trade_price,
                        candle_acc_trade_volume = EXCLUDED.candle_acc_trade_volume,
                        unit = EXCLUDED.unit
                    "#,
                )
                .bind(market)
                .bind(candle_date_time_utc)
                .bind(candle_date_time_kst)
                .bind(opening_price)
                .bind(high_price)
                .bind(low_price)
                .bind(trade_price)
                .bind(candle_acc_trade_price)
                .bind(candle_acc_trade_volume)
                .bind(unit)
                .execute(pool).await
                .map_err(|e| {
                    error!(table, error = e.to_string(), market, "Failed to insert candle to candles_minutes");
                    e
                })?;
            }
        }
    }

    Ok(())
}
