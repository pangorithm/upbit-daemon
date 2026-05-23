use crate::api::quotation::candle;
use crate::config::Config;
use chrono::Duration;
use serde_json::Value;
use sqlx::PgPool;
use sqlx::Row;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
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
        unit: String,
    },
    CandlesDays {
        market: String,
        to: String,
        count: u32,
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

fn candle_table_for_unit(unit: &str) -> &'static str {
    if crate::api::quotation::candle::is_seconds_unit(unit) {
        "candles_seconds"
    } else if crate::api::quotation::candle::is_days_unit(unit) {
        "candles_days"
    } else {
        "candles_minutes"
    }
}

pub async fn fill_candle_minutes_gap(
    pool: &PgPool,
    rest: &crate::api::rest::RestClient,
    config: &Config,
    market: &str,
    unit: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let last_candle_time = get_last_candle_time(pool, market, unit).await?;

    let api_unit = crate::api::quotation::candle::unit_to_api_value(unit);
    let numeric_unit: u32 = api_unit.parse().unwrap_or(1);

    match last_candle_time {
        Some(last) => {
            let gap_minutes = calculate_gap_minutes(Some(&last))?;
            if gap_minutes == 0 {
                info!(market, unit, "No gap in candle data");
                return Ok(());
            }

            info!(
                market,
                gap_minutes, unit, "Adding gap-filling to global queue"
            );

            let queue = get_global_queue();
            let total_candles_needed = gap_minutes / numeric_unit;
            let mut remaining_candles = total_candles_needed;

            let mut current_from = last
                .parse::<chrono::DateTime<chrono::Utc>>()
                .expect("Failed to parse last_candle_time");

            while remaining_candles > 0 {
                let batch_size = std::cmp::min(remaining_candles, config.candle.count);
                let to_str = current_from
                    .checked_add_signed(Duration::minutes((batch_size * numeric_unit) as i64))
                    .expect("Time overflow")
                    .format("%Y-%m-%dT%H:%M:%S+00:00")
                    .to_string();
                queue.lock().await.push_back(ApiRequestDto::CandlesMinutes {
                    market: market.to_string(),
                    count: batch_size,
                    to: to_str,
                    unit: unit.to_string(),
                });
                current_from = current_from
                    .checked_add_signed(Duration::minutes((batch_size * numeric_unit) as i64))
                    .expect("Time overflow");
                remaining_candles -= batch_size;
            }

            start_background_task(queue, pool.clone(), rest.clone(), config.clone());
            info!(market, unit, "Candle gap filled successfully");
        }
        None => {
            // No last candle: fetch count candles from current time
            info!(
                market,
                unit,
                count = config.candle.count,
                "No last candle found, fetching {} candles from current time",
                config.candle.count
            );
            let queue = get_global_queue();
            let now = chrono::Utc::now();
            let to_str = now.format("%Y-%m-%dT%H:%M:%S+00:00").to_string();
            queue.lock().await.push_back(ApiRequestDto::CandlesMinutes {
                market: market.to_string(),
                count: config.candle.count,
                to: to_str,
                unit: unit.to_string(),
            });
            start_background_task(queue, pool.clone(), rest.clone(), config.clone());
            info!(market, unit, "Initial candle fetch queued");
        }
    }

    Ok(())
}

pub async fn fill_candle_days_gap(
    pool: &PgPool,
    rest: &crate::api::rest::RestClient,
    config: &Config,
    market: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let last_candle_date = get_last_candle_days_time(pool, market).await?;

    match last_candle_date {
        Some(last) => {
            let last_time = chrono::NaiveDateTime::parse_from_str(&last, "%Y-%m-%dT%H:%M:%S")
                .map_err(|e| format!("Failed to parse last candle time '{}': {}", last, e))?;
            let now = chrono::Utc::now().naive_utc();
            let diff_days = (now - last_time).num_days();

            if diff_days <= 0 {
                info!(market, "No gap in daily candle data");
                return Ok(());
            }

            info!(
                market,
                diff_days, "Adding daily candle gap-filling to global queue"
            );

            let queue = get_global_queue();
            let mut remaining_candles = diff_days as u32;

            while remaining_candles > 0 {
                let batch_size = std::cmp::min(remaining_candles, config.candle.count);
                let to_str = chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::days(batch_size as i64))
                    .expect("Time overflow")
                    .format("%Y-%m-%d")
                    .to_string();
                queue.lock().await.push_back(ApiRequestDto::CandlesDays {
                    market: market.to_string(),
                    to: to_str,
                    count: batch_size,
                });
                remaining_candles -= batch_size;
            }

            start_background_task(queue, pool.clone(), rest.clone(), config.clone());
            info!(market, "Daily candle gap filled successfully");
        }
        None => {
            // No last candle: fetch count candles from current time
            info!(
                market,
                count = config.candle.count,
                "No last candle found for days, fetching {} candles from current time",
                config.candle.count
            );
            let queue = get_global_queue();
            let to_str = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(1))
                .expect("Time overflow")
                .format("%Y-%m-%d")
                .to_string();
            queue.lock().await.push_back(ApiRequestDto::CandlesDays {
                market: market.to_string(),
                to: to_str,
                count: config.candle.count,
            });

            start_background_task(queue, pool.clone(), rest.clone(), config.clone());
            info!(market, "Initial daily candle fetch queued");
        }
    }

    Ok(())
}

pub async fn fill_all_candle_gaps(
    pool: &PgPool,
    rest: &crate::api::rest::RestClient,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let markets_rows = sqlx::query(r#"SELECT market FROM markets ORDER BY market"#)
        .fetch_all(pool)
        .await?;
    let all_markets: Vec<String> = markets_rows
        .iter()
        .map(|r: &sqlx::postgres::PgRow| r.get("market"))
        .collect();

    let markets: Vec<&String> = match &config.candle.market_prefix {
        Some(prefix) => all_markets
            .iter()
            .filter(|m| m.starts_with(prefix))
            .collect(),
        None => all_markets.iter().collect(),
    };

    for market in &markets {
        for unit in &config.candle.units {
            if crate::api::quotation::candle::is_days_unit(unit) {
                if let Err(e) = fill_candle_days_gap(pool, rest, config, market).await {
                    error!(market, unit, error = %e, "Failed to fill candles days gap");
                }
            } else {
                if let Err(e) = fill_candle_minutes_gap(pool, rest, config, market, unit).await {
                    error!(market, unit, error = %e, "Failed to fill candle gap");
                }
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

    HANDLE
        .get_or_init(|| {
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
                                    &rest_client,
                                    &market,
                                    &unit,
                                    count,
                                    &to,
                                )
                                .await
                                {
                                    Ok(candles) => {
                                        if let Err(e) = insert_candles(&pool, &candles, &unit).await {
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
                            ApiRequestDto::CandlesDays {
                                market,
                                to,
                                count,
                            } => {
                                match candle::get_candles_days(
                                    &rest_client,
                                    &market,
                                    &to,
                                    count,
                                )
                                .await
                                {
                                    Ok(candles) => {
                                        if let Err(e) = insert_candles(&pool, &candles, "1d").await {
                                            error!(
                                                api_type = "CandlesDays",
                                                market,
                                                error = e.to_string(),
                                                "Failed to insert daily candles from background task"
                                            );
                                        } else {
                                            info!(
                                                api_type = "CandlesDays",
                                                market,
                                                count = candles.len(),
                                                "Batch daily candles fetched and inserted via global queue"
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        error!(
                                            api_type = "CandlesDays",
                                            market,
                                            count,
                                            error = e.to_string(),
                                            "Failed to fetch daily candles"
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
        })
        .clone()
}

async fn get_last_candle_time(
    pool: &PgPool,
    market: &str,
    unit: &str,
) -> Result<Option<String>, sqlx::Error> {
    let table = candle_table_for_unit(unit);
    let query = match table {
        "candles_days" => {
            r#"SELECT candle_date_time_utc FROM candles_days WHERE market = $1 ORDER BY candle_date_time_utc DESC LIMIT 1"#
        }
        _ => {
            r#"SELECT candle_date_time_utc FROM candles_minutes WHERE market = $1 AND unit = $2 ORDER BY candle_date_time_utc DESC LIMIT 1"#
        }
    };

    match table {
        "candles_days" => sqlx::query(query)
            .bind(market)
            .fetch_optional(pool)
            .await
            .map(|row| row.map(|r: sqlx::postgres::PgRow| r.get("candle_date_time_utc"))),
        _ => {
            let api_unit = crate::api::quotation::candle::unit_to_api_value(unit);
            sqlx::query(query)
                .bind(market)
                .bind(api_unit.parse::<i32>().unwrap_or(10))
                .fetch_optional(pool)
                .await
                .map(|row| row.map(|r: sqlx::postgres::PgRow| r.get("candle_date_time_utc")))
        }
    }
}

async fn get_last_candle_days_time(
    pool: &PgPool,
    market: &str,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query(
        r#"SELECT candle_date_time_utc FROM candles_days WHERE market = $1 ORDER BY candle_date_time_utc DESC LIMIT 1"#,
    )
    .bind(market)
    .fetch_optional(pool)
    .await
    .map(|row| row.map(|r: sqlx::postgres::PgRow| r.get("candle_date_time_utc")))
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
    unit: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let table = candle_table_for_unit(unit);

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

        match table {
            "candles_days" => {
                let timestamp = candle["timestamp"].as_i64().unwrap_or(0);
                let prev_closing_price = candle["prev_closing_price"].as_f64();
                let change_price = candle["change_price"].as_f64();
                let change_rate = candle["change_rate"].as_f64();
                let converted_trade_price = candle["converted_trade_price"].as_f64();

                sqlx::query(
                    r#"
                    INSERT INTO candles_days (
                        market, candle_date_time_utc, candle_date_time_kst,
                        opening_price, high_price, low_price, trade_price,
                        timestamp, candle_acc_trade_price, candle_acc_trade_volume,
                        prev_closing_price, change_price, change_rate, converted_trade_price
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                    ON CONFLICT (market, candle_date_time_utc) DO UPDATE SET
                        candle_date_time_kst = EXCLUDED.candle_date_time_kst,
                        opening_price = EXCLUDED.opening_price,
                        high_price = EXCLUDED.high_price,
                        low_price = EXCLUDED.low_price,
                        trade_price = EXCLUDED.trade_price,
                        timestamp = EXCLUDED.timestamp,
                        candle_acc_trade_price = EXCLUDED.candle_acc_trade_price,
                        candle_acc_trade_volume = EXCLUDED.candle_acc_trade_volume,
                        prev_closing_price = EXCLUDED.prev_closing_price,
                        change_price = EXCLUDED.change_price,
                        change_rate = EXCLUDED.change_rate,
                        converted_trade_price = EXCLUDED.converted_trade_price
                    "#,
                )
                .bind(market)
                .bind(&candle_date_time_utc)
                .bind(&candle_date_time_kst)
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
                .execute(pool)
                .await
                .map_err(|e| {
                    error!(
                        table,
                        error = e.to_string(),
                        market,
                        "Failed to insert candle to candles_days"
                    );
                    e
                })?;
            }
            _ => {
                let timestamp = candle["timestamp"].as_i64().unwrap_or(0);
                let unit_val = candle["unit"].as_u64().unwrap_or(0) as i64;
                sqlx::query(
                    r#"
                    INSERT INTO candles_minutes (
                        market, candle_date_time_utc, candle_date_time_kst,
                        opening_price, high_price, low_price, trade_price,
                        timestamp, candle_acc_trade_price, candle_acc_trade_volume, unit
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    ON CONFLICT (market, candle_date_time_utc, unit) DO UPDATE SET
                        candle_date_time_kst = EXCLUDED.candle_date_time_kst,
                        opening_price = EXCLUDED.opening_price,
                        high_price = EXCLUDED.high_price,
                        low_price = EXCLUDED.low_price,
                        trade_price = EXCLUDED.trade_price,
                        timestamp = EXCLUDED.timestamp,
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
                .bind(timestamp)
                .bind(candle_acc_trade_price)
                .bind(candle_acc_trade_volume)
                .bind(unit_val)
                .execute(pool)
                .await
                .map_err(|e| {
                    error!(
                        table,
                        error = e.to_string(),
                        market,
                        "Failed to insert candle to candles_minutes"
                    );
                    e
                })?;
            }
        }
    }

    Ok(())
}

struct GapFillingGuard<'a>(&'a AtomicBool);

impl<'a> Drop for GapFillingGuard<'a> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

pub async fn run_gap_filling(pool: &PgPool, rest: &crate::api::rest::RestClient, config: &Config) {
    info!("Starting candle gap-filling");

    let running = AtomicBool::new(false);

    if let Err(e) = fill_all_candle_gaps(pool, rest, config).await {
        error!("Initial candle gap-filling failed: {}", e);
    }

    loop {
        let next = crate::cron::interval::next_cron_instant(
            config.cron.candle.as_deref(),
            tokio::time::Instant::now() + std::time::Duration::from_secs(600),
        );
        tokio::time::sleep_until(next).await;

        if running
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            let _guard = GapFillingGuard(&running); // 작업 완료/패닉 시 자동으로 running을 false로 변경
            if let Err(e) = fill_all_candle_gaps(pool, rest, config).await {
                error!("Candle gap-filling failed: {}", e);
            }
            running.store(false, Ordering::Release);
        } else {
            info!("Skipping candle gap-filling (previous run still in progress)");
        }
    }
}
