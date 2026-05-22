use serde_json::json;
use sqlx::{PgPool, Row};
use tracing::info;
use crate::config::ApiConfig;

pub async fn subscribe_markets(
    pool: &PgPool,
    _config: &ApiConfig,
    send: impl Fn(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rows = sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(pool).await?;
    let markets: Vec<String> = rows.iter().map(|r: &sqlx::postgres::PgRow| r.get("market")).collect();

    if markets.is_empty() {
        info!("No markets found, skipping subscription");
        return Ok(());
    }

    info!("Subscribing to {} markets", markets.len());

    let ticker_msg = json!([
        {
            "format": "SIMPLE",
            "type": "ticker",
            "codes": markets
        }
    ]);

    send(&ticker_msg.to_string()).await;

    let candle_msg = json!([
        {
            "code": "KRW-BTC",
            "name": "candle",
            "tickerSymbol": {
                "market": "KRW-BTC",
                "tickerIntervalType": "minutes10"
            }
        }
    ]);

    send(&candle_msg.to_string()).await;

    Ok(())
}

pub async fn subscribe_new_market(
    _pool: &PgPool,
    market: &str,
    send: impl Fn(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Dynamically subscribing to new market: {}", market);

    let ticker_msg = json!([
        {
            "format": "SIMPLE",
            "type": "ticker",
            "codes": [market]
        }
    ]);

    send(&ticker_msg.to_string()).await;

    Ok(())
}
