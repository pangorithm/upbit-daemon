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

    // Subscribe to minute/day candles from all markets
    for unit in &config.candle.units {
        let api_unit = crate::api::quotation::candle::unit_to_api_value(unit);
        let ws_type = if crate::api::quotation::candle::is_days_unit(unit) {
            "candle.240m"
        } else {
            format!("candle.{}m", api_unit).leak()
        };
        let candle_msg = json!([
            {"ticket": uuid::Uuid::new_v4().to_string()},
            {"type": ws_type, "codes": markets.clone()},
            {"format": "DEFAULT"}
        ]);
        if let Err(e) = ws.send(
            tokio_tungstenite::tungstenite::Message::Text(candle_msg.to_string().into())
        ).await {
            error!("Failed to subscribe to {} for {} markets: {}", ws_type, markets.len(), e);
        } else {
            info!("Subscribed to {} markets for {} units", markets.len(), ws_type);
        }
    }

    // Subscribe to 1s candles from explicitly listed markets only
    for market in &config.candle.seconds.markets {
        let candle_msg = json!([
            {"ticket": uuid::Uuid::new_v4().to_string()},
            {"type": "candle.1s", "codes": [market.as_str()]},
            {"format": "DEFAULT"}
        ]);
        if let Err(e) = ws.send(
            tokio_tungstenite::tungstenite::Message::Text(candle_msg.to_string().into())
        ).await {
            error!("Failed to subscribe to candle.1s for market {}: {}", market, e);
        } else {
            info!("Subscribed to market {} for candle.1s unit", market);
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

    for unit in &config.candle.units {
        let api_unit = crate::api::quotation::candle::unit_to_api_value(unit);
        let ws_type = if crate::api::quotation::candle::is_days_unit(unit) {
            "candle.240m"
        } else {
            format!("candle.{}m", api_unit).leak()
        };
        let candle_msg = json!([
            {"ticket": uuid::Uuid::new_v4().to_string()},
            {"type": ws_type, "codes": [market]},
            {"format": "DEFAULT"}
        ]);
        if let Err(e) = ws.send(
            tokio_tungstenite::tungstenite::Message::Text(candle_msg.to_string().into())
        ).await {
            error!("Failed to subscribe to {} for market {}: {}", ws_type, market, e);
        } else {
            info!("Subscribed to market {} for {} unit", market, ws_type);
        }
    }

    Ok(())
}
