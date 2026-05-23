use std::collections::HashSet;

use sqlx::PgPool;


use crate::ws::client::WebSocketClient;
use crate::config;

use super::session::find_and_subscribe_new;

pub async fn handle_subscription_update(
    _pool: &PgPool,
    _ws: &WebSocketClient,
    config: &config::Config,
    _msg: &serde_json::Value,
    subscribed_markets: &'static std::sync::Mutex<HashSet<String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let candle_markets = find_and_subscribe_new(_ws, "1s", config, &config.subscribe.candle);
    let ticker_markets = find_and_subscribe_new(_ws, "ticker", config, &config.subscribe.ticker);
    let trade_markets = find_and_subscribe_new(_ws, "trade", config, &config.subscribe.trade);
    let orderbook_markets = find_and_subscribe_new(_ws, "orderbook", config, &config.subscribe.orderbook);

    let mut lock = subscribed_markets.lock().unwrap();
    for market in &candle_markets {
        lock.insert(market.clone());
    }
    for market in &ticker_markets {
        lock.insert(market.clone());
    }
    for market in &trade_markets {
        lock.insert(market.clone());
    }
    for market in &orderbook_markets {
        lock.insert(market.clone());
    }

    Ok(())
}
