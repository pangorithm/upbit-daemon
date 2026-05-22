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

    info!("Subscribing to {} markets for candle tick", markets.len());

    let candle_msg = json!([
        {
            "format": "DEFAULT",
            "type": "tick",
            "codes": markets
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

    let candle_msg = json!([
        {
            "format": "DEFAULT",
            "type": "tick",
            "codes": [market]
        }
    ]);

    send(&candle_msg.to_string()).await;

    Ok(())
}
