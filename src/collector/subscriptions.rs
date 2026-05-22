use serde_json::json;
use sqlx::{PgPool, Row};
use tracing::info;
use crate::config::Config;

pub async fn subscribe_markets(
    pool: &PgPool,
    config: &Config,
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

    info!("Subscribing to {} markets for candle units {:?}", markets.len(), config.candle.units);

    // 첫 번째 candle unit으로 구독 (config.candle.units[0])
    let candle_msg = json!([
        {
            "format": "DEFAULT",
            "type": format!("candle.{}", config.candle.units[0]),
            "codes": markets
        }
    ]);

    send(&candle_msg.to_string()).await;

    Ok(())
}

pub async fn subscribe_new_market(
    config: &Config,
    market: &str,
    send: impl Fn(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Dynamically subscribing to new market: {} with units {:?}", market, config.candle.units);

    let candle_msg = json!([
        {
            "format": "DEFAULT",
            "type": format!("candle.{}", config.candle.units[0]),
            "codes": [market]
        }
    ]);

    send(&candle_msg.to_string()).await;

    Ok(())
}
