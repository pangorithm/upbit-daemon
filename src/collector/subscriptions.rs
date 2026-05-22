use serde_json::json;
use sqlx::{PgPool, Row};
use tracing::{error, info};
use crate::config::Config;
use crate::api::websocket::WebSocketClient;

pub async fn subscribe_markets(
    pool: &PgPool,
    ws: &WebSocketClient,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rows = sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(pool).await?;
    let markets: Vec<String> = rows.iter().map(|r: &sqlx::postgres::PgRow| r.get("market")).collect();

    if markets.is_empty() {
        info!("No markets found, skipping subscription");
        return Ok(());
    }

    for &unit in &config.candle.units {
        let candle_msg = json!([
            {"ticket": uuid::Uuid::new_v4().to_string()},
            {"type": format!("candle.{}", unit), "codes": markets.clone()},
            {"format": "DEFAULT"}
        ]);
        if let Err(e) = ws.send(
            tokio_tungstenite::tungstenite::Message::Text(candle_msg.to_string().into())
        ).await {
            error!("Failed to subscribe to candle.{} for {} markets: {}", unit, markets.len(), e);
        } else {
            info!("Subscribed to {} markets for candle.{} units", markets.len(), unit);
        }
    }

    Ok(())
}

pub async fn subscribe_new_market(
    ws: &WebSocketClient,
    config: &Config,
    market: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Dynamically subscribing to new market: {} with units {:?}", market, config.candle.units);

    for &unit in &config.candle.units {
        let candle_msg = json!([
            {"ticket": uuid::Uuid::new_v4().to_string()},
            {"type": format!("candle.{}", unit), "codes": [market]},
            {"format": "DEFAULT"}
        ]);
        if let Err(e) = ws.send(
            tokio_tungstenite::tungstenite::Message::Text(candle_msg.to_string().into())
        ).await {
            error!("Failed to subscribe to candle.{} for market {}: {}", unit, market, e);
        } else {
            info!("Subscribed to market {} for candle.{} unit", market, unit);
        }
    }

    Ok(())
}
