use serde_json::json;
use sqlx::{PgPool, Row};
use tracing::{error, info};
use crate::config::Config;
use crate::api::websocket::WebSocketClient;

/// Maximum codes per WebSocket subscription message (Upbit API limit)
const MAX_CODES_PER_SUBSCRIPTION: usize = 10;
/// Delay between subscription batch sends
const SUBSCRIBE_BATCH_DELAY: std::time::Duration = std::time::Duration::from_millis(200);

fn is_days_unit(unit: &str) -> bool {
    crate::api::quotation::candle::is_days_unit(unit)
}

fn unit_to_api_value(unit: &str) -> &str {
    crate::api::quotation::candle::unit_to_api_value(unit)
}

/// Subscribe to all markets for the specified units.
/// Returns the units that were successfully subscribed (up to the first failure).
/// `units_to_subscribe` filters which candle units to process (empty = all).
pub async fn subscribe_markets(
    pool: &PgPool,
    ws: &WebSocketClient,
    config: &Config,
    units_to_subscribe: &[&String],
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let rows = sqlx::query(
        r#"SELECT market FROM markets ORDER BY market"#
    ).fetch_all(pool).await?;
    let all_markets: Vec<String> = rows.iter().map(|r: &sqlx::postgres::PgRow| r.get("market")).collect();
    let total_count = all_markets.len();

    let markets: Vec<String> = match &config.candle.market_prefix {
        Some(prefix) => all_markets.iter().filter(|m| m.starts_with(prefix)).cloned().collect(),
        None => all_markets,
    };

    if markets.is_empty() {
        info!("No markets found matching prefix, skipping subscription");
        return Ok(Vec::new());
    }

    info!(
        "Subscribing to {} markets (of {} total) with prefix {:?}",
        markets.len(),
        total_count,
        config.candle.market_prefix
    );

    let units = if units_to_subscribe.is_empty() {
        config
            .candle
            .units
            .iter()
            .map(|u| u.as_str())
            .collect::<Vec<_>>()
    } else {
        units_to_subscribe
            .iter()
            .map(|u| u.as_str())
            .collect::<Vec<_>>()
    };

    let mut subscribed_units = Vec::new();

    for unit in &units {
        let api_unit = unit_to_api_value(unit);
        let ws_type = if is_days_unit(unit) {
            "candle.1d"
        } else {
            format!("candle.{}m", api_unit).leak()
        };

        for (i, markets_chunk) in markets.chunks(MAX_CODES_PER_SUBSCRIPTION).enumerate() {
            let candle_msg = json!([
                {"ticket": uuid::Uuid::new_v4().to_string()},
                {"type": ws_type, "codes": markets_chunk},
                {"format": "DEFAULT"}
            ]);
            if let Err(e) = ws.send(
                tokio_tungstenite::tungstenite::Message::Text(candle_msg.to_string().into())
            ).await {
                error!(
                    "Failed to subscribe to {} batch {}/{} ({} markets): {}",
                    ws_type, i + 1, markets.chunks(MAX_CODES_PER_SUBSCRIPTION).len(), markets_chunk.len(), e
                );
                return Ok(subscribed_units);
            }
            if i < markets.chunks(MAX_CODES_PER_SUBSCRIPTION).len() - 1 {
                tokio::time::sleep(SUBSCRIBE_BATCH_DELAY).await;
                tokio::task::yield_now().await;
            }
        }
        info!("Subscribed to all {} markets for {}", markets.len(), ws_type);
        subscribed_units.push(unit.to_string());
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
            error!(
                "Failed to subscribe to candle.1s for market {}: {}",
                market, e
            );
        } else {
            info!("Subscribed to market {} for candle.1s unit", market);
        }
    }

    Ok(subscribed_units)
}

pub async fn subscribe_new_market(
    ws: &WebSocketClient,
    config: &Config,
    market: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Skip markets that don't match the configured prefix
    if let Some(prefix) = &config.candle.market_prefix {
        if !market.starts_with(prefix) {
            return Ok(());
        }
    }

    info!(
        "Dynamically subscribing to new market: {} with units {:?}",
        market, config.candle.units
    );

    for unit in &config.candle.units {
        let api_unit = unit_to_api_value(unit);
        let ws_type = if is_days_unit(unit) {
            "candle.1d"
        } else {
            format!("candle.{}m", api_unit).leak()
        };
        let candle_msg = json!([
            {"ticket": uuid::Uuid::new_v4().to_string()},
            {"type": ws_type, "codes": [market]},
            {"format": "DEFAULT"}
        ]);
        if let Err(e) = ws
            .send(tokio_tungstenite::tungstenite::Message::Text(
                candle_msg.to_string().into(),
            ))
            .await
        {
            error!(
                "Failed to subscribe to {} for market {}: {}",
                ws_type, market, e
            );
        } else {
            info!("Subscribed to market {} for {} unit", market, ws_type);
        }
    }

    Ok(())
}
