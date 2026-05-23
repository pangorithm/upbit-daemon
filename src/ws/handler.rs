use std::collections::HashSet;

use sqlx::PgPool;
use tracing::info;

use crate::api::websocket::WebSocketClient;
use crate::config;

use super::session::find_and_subscribe_new;

pub async fn handle_subscription_update(
    pool: &PgPool,
    ws: &WebSocketClient,
    config: &config::Config,
    msg: &serde_json::Value,
    subscribed_units: &'static std::sync::Mutex<HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscribed_codes = msg.as_array().and_then(|arr| {
        arr.iter().find_map(|item| {
            item.get("codes").and_then(|codes| codes.as_array())
                .map(|c| c.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
        })
    });

    let subscribed: HashSet<String> = match subscribed_codes {
        Some(codes) => codes.into_iter().collect(),
        None => std::collections::HashSet::new(),
    };

    let filtered = find_and_subscribe_new(pool, ws, config, &subscribed).await?;
    if !filtered.is_empty() {
        info!("Found {} new markets, subscribing...", filtered.len());
        let mut lock = subscribed_units.lock().unwrap();
        for market in &filtered {
            lock.insert(market.clone());
        }
    }

    Ok(())
}
