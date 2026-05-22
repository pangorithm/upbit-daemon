use serde_json::Value;
use sqlx::PgPool;
use tracing::{error, info};

pub async fn handle_message(pool: &PgPool, msg: &Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let msg_type = msg["type"].as_str().unwrap_or("");
    match msg_type {
        "tick" => handle_candle_tick(pool, msg).await,
        "trade" => handle_trade_msg(pool, msg).await,
        "ticker" => handle_ticker_msg(pool, msg).await,
        "orderbook" => handle_orderbook_msg(pool, msg).await,
        _ => Ok(()),
    }
}

async fn handle_candle_tick(pool: &PgPool, msg: &Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market = msg["code"].as_str().unwrap_or("");
    let candle = &msg["candle"];

    let candle_date_time_utc = candle["candle_date_time_utc"].as_str().unwrap_or("");
    let candle_date_time_kst = candle["candle_date_time_kst"].as_str().unwrap_or("");
    let opening_price = candle["opening_price"].as_f64().unwrap_or(0.0);
    let high_price = candle["high_price"].as_f64().unwrap_or(0.0);
    let low_price = candle["low_price"].as_f64().unwrap_or(0.0);
    let trade_price = candle["trade_price"].as_f64().unwrap_or(0.0);
    let candle_acc_trade_price = candle["candle_acc_trade_price"].as_f64().unwrap_or(0.0);
    let candle_acc_trade_volume = candle["candle_acc_trade_volume"].as_f64().unwrap_or(0.0);
    let unit = candle["unit"].as_i64().unwrap_or(1);

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO candles_minutes (market, candle_date_time_utc, candle_date_time_kst,
            opening_price, high_price, low_price, trade_price,
            candle_acc_trade_price, candle_acc_trade_volume, unit)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
    )
    .bind(market)
    .bind(candle_date_time_utc)
    .bind(candle_date_time_kst)
    .bind(opening_price)
    .bind(high_price)
    .bind(low_price)
    .bind(trade_price)
    .bind(candle_acc_trade_price)
    .bind(candle_acc_trade_volume)
    .bind(unit)
    .execute(pool)
    .await {
        error!(market = market, error = %e, "Failed to upsert candle");
    } else {
        info!(market = market, utc = candle_date_time_utc, "Candle upserted");
    }

    Ok(())
}

async fn handle_trade_msg(pool: &PgPool, msg: &Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market = msg["code"].as_str().unwrap_or("");
    let trade_price = msg["trade_price"].as_f64().unwrap_or(0.0);
    let trade_volume = msg["trade_volume"].as_f64().unwrap_or(0.0);
    let sequential_id = msg["sequential_id"].as_i64().unwrap_or(0);
    let timestamp = msg["timestamp"].as_i64().unwrap_or(0);

    let trade_date_utc = msg["trade_date_utc"].as_str().unwrap_or("");
    let trade_time_utc = msg["trade_time_utc"].as_str().unwrap_or("");

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO trades (market, trade_date_utc, trade_time_utc, trade_price,
            trade_volume, sequential_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (sequential_id) DO NOTHING
        "#,
    )
    .bind(market)
    .bind(trade_date_utc)
    .bind(trade_time_utc)
    .bind(trade_price)
    .bind(trade_volume)
    .bind(sequential_id)
    .execute(pool)
    .await {
        error!(market = market, error = %e, "Failed to insert trade");
    }

    Ok(())
}

async fn handle_ticker_msg(pool: &PgPool, msg: &Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market = msg["code"].as_str().unwrap_or("");
    let trade_date = msg["trade_date"].as_str().unwrap_or("");
    let trade_time = msg["trade_time"].as_str().unwrap_or("");
    let trade_date_kst = msg["trade_date_kst"].as_str().unwrap_or("");
    let trade_time_kst = msg["trade_time_kst"].as_str().unwrap_or("");
    let trade_timestamp = msg["timestamp"].as_i64().unwrap_or(0);
    let opening_price = msg["opening_price"].as_f64().unwrap_or(0.0);
    let high_price = msg["high_price"].as_f64().unwrap_or(0.0);
    let low_price = msg["low_price"].as_f64().unwrap_or(0.0);
    let trade_price = msg["trade_price"].as_f64().unwrap_or(0.0);
    let prev_closing_price = msg["prev_closing_price"].as_f64().unwrap_or(0.0);
    let change = msg["change"].as_str().unwrap_or("EVEN");
    let change_price = msg["change_price"].as_f64().unwrap_or(0.0);
    let change_rate = msg["change_rate"].as_f64().unwrap_or(0.0);
    let signed_change_price = msg["signed_change_price"].as_f64().unwrap_or(0.0);
    let signed_change_rate = msg["signed_change_rate"].as_f64().unwrap_or(0.0);
    let trade_volume = msg["trade_volume"].as_f64().unwrap_or(0.0);
    let acc_trade_price = msg["acc_trade_price"].as_f64().unwrap_or(0.0);
    let acc_trade_price_24h = msg["acc_trade_price_24h"].as_f64().unwrap_or(0.0);
    let acc_trade_volume = msg["acc_trade_volume"].as_f64().unwrap_or(0.0);
    let acc_trade_volume_24h = msg["acc_trade_volume_24h"].as_f64().unwrap_or(0.0);
    let highest_52_week_price = msg["highest_52_week_price"].as_f64().unwrap_or(0.0);
    let lowest_52_week_price = msg["lowest_52_week_price"].as_f64().unwrap_or(0.0);
    let timestamp = msg["timestamp"].as_i64().unwrap_or(0);

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO tickers (market, trade_date, trade_time, trade_date_kst, trade_time_kst,
            trade_timestamp, opening_price, high_price, low_price, trade_price,
            prev_closing_price, change, change_price, change_rate,
            signed_change_price, signed_change_rate, trade_volume,
            acc_trade_price, acc_trade_price_24h, acc_trade_volume,
            acc_trade_volume_24h, highest_52_week_price, lowest_52_week_price, timestamp)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
        ON CONFLICT (market, trade_date, trade_time) DO UPDATE SET
            trade_date_kst = EXCLUDED.trade_date_kst,
            trade_time_kst = EXCLUDED.trade_time_kst,
            trade_timestamp = EXCLUDED.trade_timestamp,
            opening_price = EXCLUDED.opening_price,
            high_price = EXCLUDED.high_price,
            low_price = EXCLUDED.low_price,
            trade_price = EXCLUDED.trade_price,
            prev_closing_price = EXCLUDED.prev_closing_price,
            change = EXCLUDED.change,
            change_price = EXCLUDED.change_price,
            change_rate = EXCLUDED.change_rate,
            signed_change_price = EXCLUDED.signed_change_price,
            signed_change_rate = EXCLUDED.signed_change_rate,
            trade_volume = EXCLUDED.trade_volume,
            acc_trade_price = EXCLUDED.acc_trade_price,
            acc_trade_price_24h = EXCLUDED.acc_trade_price_24h,
            acc_trade_volume = EXCLUDED.acc_trade_volume,
            acc_trade_volume_24h = EXCLUDED.acc_trade_volume_24h,
            highest_52_week_price = EXCLUDED.highest_52_week_price,
            lowest_52_week_price = EXCLUDED.lowest_52_week_price,
            timestamp = EXCLUDED.timestamp
        "#,
    )
    .bind(market)
    .bind(trade_date)
    .bind(trade_time)
    .bind(trade_date_kst)
    .bind(trade_time_kst)
    .bind(trade_timestamp)
    .bind(opening_price)
    .bind(high_price)
    .bind(low_price)
    .bind(trade_price)
    .bind(prev_closing_price)
    .bind(change)
    .bind(change_price)
    .bind(change_rate)
    .bind(signed_change_price)
    .bind(signed_change_rate)
    .bind(trade_volume)
    .bind(acc_trade_price)
    .bind(acc_trade_price_24h)
    .bind(acc_trade_volume)
    .bind(acc_trade_volume_24h)
    .bind(highest_52_week_price)
    .bind(lowest_52_week_price)
    .bind(timestamp)
    .execute(pool)
    .await {
        error!(market = market, error = %e, "Failed to upsert ticker");
    }

    Ok(())
}

async fn handle_orderbook_msg(pool: &PgPool, msg: &Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market = msg["code"].as_str().unwrap_or("");
    let timestamp = msg["timestamp"].as_i64().unwrap_or(0);
    let total_ask_size = msg["total_ask_size"].as_f64().unwrap_or(0.0);
    let total_bid_size = msg["total_bid_size"].as_f64().unwrap_or(0.0);
    let orderbook_units = msg["orderbook_units"].clone();

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO orderbooks (market, timestamp, total_ask_size, total_bid_size, orderbook_units)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(market)
    .bind(timestamp)
    .bind(total_ask_size)
    .bind(total_bid_size)
    .bind(orderbook_units)
    .execute(pool)
    .await {
        error!(market = market, error = %e, "Failed to insert orderbook");
    }

    Ok(())
}
