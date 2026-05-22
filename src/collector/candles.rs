use super::config::ApiConfig;
use crate::api::candles_api;
use chrono::{Duration, Utc};
use serde_json::Value;
use sqlx::PgPool;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::interval;
use tracing::{error, info};

/// 전역 API 요청 DTO enum
/// 글로벌 큐가 여러 API 요청 타입을 담을 수 있음
#[derive(Debug, Clone)]
enum ApiRequestDto {
    /// 캔들 조회 요청 (분 단위)
    /// (페어명, 조회할 캔들 개수, 조회 종료 시간, 캔들 분 단위)
    CandlesMinutes {
        market: String,
        count: u32,
        to: String,
        unit: u32,
    },
}

/// 전역 API 호출 큐 타입
/// 여러 API 요청 DTO를 담을 수 있는 큐
type ApiQueue = Arc<Mutex<VecDeque<ApiRequestDto>>>;

/// 전역 큐 싱글톤 반환
/// 프로그램 전체에서 단 하나의 큐만 사용 (여러 코인 동시 gap-filling 시 공유)
fn get_global_queue() -> ApiQueue {
    use std::sync::OnceLock;
    static QUEUE: OnceLock<ApiQueue> = OnceLock::new();
    QUEUE
        .get_or_init(|| Arc::new(Mutex::new(VecDeque::new())))
        .clone()
}

/// 10분봉 캔들 gap-filling 실행
/// DB에서 마지막 캔들 시간을 조회하여 누락된 캔들을 REST API로 채움
/// - 마지막 캔들이 없으면 gap-filling 하지 않음 (새 구독)
/// - gap_minutes / candle_unit 로 필요한 캔들 개수 계산
/// - batch_size 단위로 큐에 배치 추가 (각 배치별 correct한 to 시간 계산)
/// - 백그라운드 태스크 시작 후 큐 비워질 때까지 대기
pub async fn fill_candles_minute_gap(
    pool: &PgPool,
    rest: &Client,
    config: &ApiConfig,
    market: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // DB에서 해당 페어의 마지막 캔들 시간 조회
    let last_candle_time = get_last_candle_time(pool, market).await?;

    // 마지막 캔들이 없으면 gap-filling 하지 않음 (새 구독)
    let last_candle_time = last_candle_time.ok_or("No last candle found")?;

    // 마지막 캔들 시간과 현재 시간 사이 gap(분) 계산
    let gap_minutes = calculate_gap_minutes(Some(last_candle_time))?;

    if gap_minutes == 0 {
        info!(market, unit = config.candle_unit, "No gap in candle data");
        return Ok(());
    }

    info!(
        market,
        gap_minutes,
        unit = config.candle_unit,
        "Adding gap-filling to global queue"
    );

    // 전역 큐에서 배치별 to 시간 계산 후 DTO로 큐에 추가 (과거부터 현재 순서)
    let queue = get_global_queue();
    let total_candles_needed = gap_minutes / config.candle_unit;
    let mut remaining_candles = total_candles_needed;

    // 마지막 캔들 시간부터 현재 시간까지 과거→현재 순서로 배치 생성
    let mut current_from = last_candle_time
        .parse::<chrono::DateTime<chrono::Utc>>()
        .expect("Failed to parse last_candle_time");

    while remaining_candles > 0 {
        let batch_size = std::cmp::min(remaining_candles, config.batch_size);
        let to_str = current_from
            .checked_add_signed(Duration::minutes((batch_size * config.candle_unit) as i64))
            .expect("Time overflow")
            .format("%Y-%m-%dT%H:%M:%S+00:00")
            .to_string();
        queue.lock().await.push_back(ApiRequestDto::CandlesMinutes {
            market: market.to_string(),
            count: batch_size,
            to: to_str,
            unit: config.candle_unit,
        });
        current_from = current_from
            .checked_add_signed(Duration::minutes((batch_size * config.candle_unit) as i64))
            .expect("Time overflow");
        remaining_candles -= batch_size;
    }

    // 백그라운드 태스크 시작 (이미 실행 중이면 중복 생성 안 됨)
    let handle = start_background_task(queue, pool.clone(), rest.clone(), config.clone());
    handle.await.unwrap();

    info!(market, "Gap filled successfully");

    Ok(())
}

/// 백그라운드 태스크 시작
/// - 큐에서 DTO 꺼내서 API 타입에 따라 적절한 API 호출 후 DB 저장
/// - API 호출 속도 제한 (초당 5회)
/// - 단일 태스크로 동작 (여러 코드 호출 시 공유)
fn start_background_task(
    queue: ApiQueue,
    pool: PgPool,
    rest: Client,
    config: ApiConfig,
) -> JoinHandle<()> {
    use std::sync::OnceLock;
    static HANDLE: OnceLock<JoinHandle<()>> = OnceLock::new();

    HANDLE
        .get_or_init(|| {
            tokio::spawn(async move {
                let mut timer = interval(std::time::Duration::from_secs_f64(
                    1.0 / config.api_calls_per_second as f64,
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
                                match candles_api::get_candles_minutes(
                                    &rest, &market, unit, count, &to,
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
                            break;
                        }
                    }
                }
            })
            .clone()
        })
        .clone()
}

/// DB에서 해당 페어의 마지막 캔들 시간 조회
/// candles_minutes 테이블에서 market에 맞춰 가장 최근 candle_date_time_utc 반환
async fn get_last_candle_time(pool: &PgPool, market: &str) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT candle_date_time_utc
        FROM candles_minutes
        WHERE market = $1
        ORDER BY candle_date_time_utc DESC
        LIMIT 1
        "#,
        market
    )
    .fetch_optional(pool)
    .await
}

/// 마지막 캔들 시간과 현재 시간 사이 gap(분) 계산
/// - last_candle_time이 Some이면: (now - last_candle_time).num_minutes() 반환
/// - last_candle_time이 None이면: 0 반환 (gap 없음)
/// - 음수면: 0 반환 (미래 시간인 경우)
fn calculate_gap_minutes(
    last_candle_time: Option<String>,
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

/// 캔들 데이터를 DB에 INSERT
/// REST API 응답(JSON)을 파싱하여 candles_minutes 테이블에 저장
/// - ON CONFLICT DO NOTHING: 중복 데이터 방지
/// - 10분봉 unit 고정 (config.candle_unit)
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
        let unit = 10i64;

       sqlx::query!(
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
            market,
            candle_date_time_utc,
            candle_date_time_kst,
            opening_price,
            high_price,
            low_price,
            trade_price,
            candle_acc_trade_price,
            candle_acc_trade_volume,
            unit
        )
        .execute(pool)
        .await
        .map_err(|e| {
            error!(error = e.to_string(), market, "Failed to insert candle");
            e
        })?;
    }

    Ok(())
}
