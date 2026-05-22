use serde_json::Value;
use sqlx::PgPool;
use tracing::{error, info};

/// Route WebSocket message to appropriate handler by type
/// Supported types: candle.{unit}, trade, ticker, orderbook
pub async fn handle_message(
    pool: &PgPool,
    msg: &Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let msg_type = msg["type"].as_str().unwrap_or("");
    match msg_type {
        "candle.1s" | "candle.1m" | "candle.3m" | "candle.5m" | "candle.10m" | "candle.15m"
        | "candle.30m" | "candle.60m" | "candle.240m" => {
            let market = msg["code"].as_str().unwrap_or("");
            if let Err(e) = handle_candle_msg(pool, msg).await {
                error!(type = msg_type, market, error = %e, "Failed to handle candle msg");
            }
            Ok(())
        }
        "trade" => {
            let market = msg["code"].as_str().unwrap_or("");
            if let Err(e) = handle_trade_msg(pool, msg).await {
                error!(type = msg_type, market, error = %e, "Failed to handle trade msg");
            }
            Ok(())
        }
        "ticker" => {
            let market = msg["code"].as_str().unwrap_or("");
            if let Err(e) = handle_ticker_msg(pool, msg).await {
                error!(type = msg_type, market, error = %e, "Failed to handle ticker msg");
            }
            Ok(())
        }
        "orderbook" => {
            let market = msg["code"].as_str().unwrap_or("");
            if let Err(e) = handle_orderbook_msg(pool, msg).await {
                error!(type = msg_type, market, error = %e, "Failed to handle orderbook msg");
            }
            Ok(())
        }
        _ => {
            error!(type = msg_type, "Unknown message type");
            Ok(())
        }
    }
}

/// Extract candle unit from type field (e.g. "candle.10m" → 10)
fn parse_candle_type(type_field: &str) -> &str {
    match type_field {
        "candle.1s" => "s",
        "candle.1m" => "m",
        "candle.3m" => "m",
        "candle.5m" => "m",
        "candle.10m" => "m",
        "candle.15m" => "m",
        "candle.30m" => "m",
        "candle.60m" => "m",
        "candle.240m" => "m",
        _ => "m",
    }
}

fn candle_table_for_suffix(suffix: &str) -> &'static str {
    match suffix {
        "s" => "candles_seconds",
        "d" => "candles_days",
        _ => "candles_minutes",
    }
}

fn extract_numeric_unit(type_field: &str) -> i32 {
    if let Some(rest) = type_field.strip_prefix("candle.") {
        let num_str = rest
            .strip_suffix('s')
            .or_else(|| rest.strip_suffix('m'))
            .unwrap_or(rest);
        if let Ok(n) = num_str.parse::<i32>() {
            return n;
        }
    }
    10
}

async fn handle_candle_msg(
    pool: &PgPool,
    msg: &Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market = msg["code"].as_str().unwrap_or("");
    let candle_date_time_utc = msg["candle_date_time_utc"].as_str().unwrap_or("");
    let candle_date_time_kst = msg["candle_date_time_kst"].as_str().unwrap_or("");
    let opening_price = msg["opening_price"].as_f64().unwrap_or(0.0);
    let high_price = msg["high_price"].as_f64().unwrap_or(0.0);
    let low_price = msg["low_price"].as_f64().unwrap_or(0.0);
    let trade_price = msg["trade_price"].as_f64().unwrap_or(0.0);
    let candle_acc_trade_price = msg["candle_acc_trade_price"].as_f64().unwrap_or(0.0);
    let candle_acc_trade_volume = msg["candle_acc_trade_volume"].as_f64().unwrap_or(0.0);
    let suffix = parse_candle_type(msg["type"].as_str().unwrap_or("candle.10m"));
    let table = candle_table_for_suffix(suffix);
    let numeric_unit = extract_numeric_unit(msg["type"].as_str().unwrap_or("candle.10m"));

    let query = match table {
        "candles_seconds" => {
            r#"
            INSERT INTO candles_seconds (market, candle_date_time_utc, candle_date_time_kst,
                opening_price, high_price, low_price, trade_price,
                timestamp, candle_acc_trade_price, candle_acc_trade_volume)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (market, candle_date_time_utc) DO UPDATE SET
                candle_date_time_kst = EXCLUDED.candle_date_time_kst,
                opening_price = EXCLUDED.opening_price,
                high_price = EXCLUDED.high_price,
                low_price = EXCLUDED.low_price,
                trade_price = EXCLUDED.trade_price,
                timestamp = EXCLUDED.timestamp,
                candle_acc_trade_price = EXCLUDED.candle_acc_trade_price,
                candle_acc_trade_volume = EXCLUDED.candle_acc_trade_volume
            "#
        }
        "candles_days" => {
            r#"
            INSERT INTO candles_days (market, candle_date_time_utc, candle_date_time_kst,
                opening_price, high_price, low_price, trade_price,
                timestamp, candle_acc_trade_price, candle_acc_trade_volume,
                prev_closing_price, change_price, change_rate, converted_trade_price)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (market, candle_date_time_utc) DO UPDATE SET
                candle_date_time_kst = EXCLUDED.candle_date_time_kst,
                opening_price = EXCLUDED.opening_price,
                high_price = EXCLUDED.high_price,
                low_price = EXCLUDED.low_price,
                trade_price = EXCLUDED.trade_price,
                timestamp = EXCLUDED.timestamp,
                candle_acc_trade_price = EXCLUDED.candle_acc_trade_price,
                candle_acc_trade_volume = EXCLUDED.candle_acc_trade_volume,
                prev_closing_price = EXCLUDED.prev_closing_price,
                change_price = EXCLUDED.change_price,
                change_rate = EXCLUDED.change_rate,
                converted_trade_price = EXCLUDED.converted_trade_price
            "#
        }
        _ => {
            r#"
            INSERT INTO candles_minutes (market, candle_date_time_utc, candle_date_time_kst,
                opening_price, high_price, low_price, trade_price,
                candle_acc_trade_price, candle_acc_trade_volume, unit)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (market, candle_date_time_utc, unit) DO UPDATE SET
                candle_date_time_kst = EXCLUDED.candle_date_time_kst,
                opening_price = EXCLUDED.opening_price,
                high_price = EXCLUDED.high_price,
                low_price = EXCLUDED.low_price,
                trade_price = EXCLUDED.trade_price,
                candle_acc_trade_price = EXCLUDED.candle_acc_trade_price,
                candle_acc_trade_volume = EXCLUDED.candle_acc_trade_volume,
                unit = EXCLUDED.unit
            "#
        }
    };

    let _rows_affected = match table {
        "candles_seconds" => {
            let timestamp = msg["timestamp"].as_i64().unwrap_or(0);
            sqlx::query(query)
                .bind(market)
                .bind(candle_date_time_utc)
                .bind(candle_date_time_kst)
                .bind(opening_price)
                .bind(high_price)
                .bind(low_price)
                .bind(trade_price)
                .bind(timestamp)
                .bind(candle_acc_trade_price)
                .bind(candle_acc_trade_volume)
                .execute(pool)
                .await
        }
        "candles_days" => {
            let timestamp = msg["timestamp"].as_i64().unwrap_or(0);
            let prev_closing_price = msg["prev_closing_price"].as_f64();
            let change_price = msg["change_price"].as_f64();
            let change_rate = msg["change_rate"].as_f64();
            let converted_trade_price = msg["converted_trade_price"].as_f64();
            sqlx::query(query)
                .bind(market)
                .bind(candle_date_time_utc)
                .bind(candle_date_time_kst)
                .bind(opening_price)
                .bind(high_price)
                .bind(low_price)
                .bind(trade_price)
                .bind(timestamp)
                .bind(candle_acc_trade_price)
                .bind(candle_acc_trade_volume)
                .bind(prev_closing_price)
                .bind(change_price)
                .bind(change_rate)
                .bind(converted_trade_price)
                .execute(pool)
                .await
        }
        _ => {
            sqlx::query(query)
                .bind(market)
                .bind(candle_date_time_utc)
                .bind(candle_date_time_kst)
                .bind(opening_price)
                .bind(high_price)
                .bind(low_price)
                .bind(trade_price)
                .bind(candle_acc_trade_price)
                .bind(candle_acc_trade_volume)
                .bind(numeric_unit)
                .execute(pool)
                .await
        }
    }
    .map_err(|e| {
        error!(table = table, market = market, error = %e, "Failed to upsert candle");
        e
    })?;

    info!(
        table = table,
        market = market,
        utc = candle_date_time_utc,
        numeric_unit,
        "Candle upserted"
    );
    Ok(())
}

/// Handle trade WebSocket message
/// WebSocket returns trade_date / trade_time (NOT trade_date_utc / trade_time_utc like REST)
async fn handle_trade_msg(
    pool: &PgPool,
    msg: &Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market = msg["code"].as_str().unwrap_or("");
    let trade_price = msg["trade_price"].as_f64().unwrap_or(0.0);
    let trade_volume = msg["trade_volume"].as_f64().unwrap_or(0.0);
    let sequential_id = msg["sequential_id"].as_i64().unwrap_or(0);

    // WebSocket field names differ from REST API
    let trade_date = msg["trade_date"].as_str().unwrap_or("");
    let trade_time = msg["trade_time"].as_str().unwrap_or("");

    sqlx::query(
        r#"
        INSERT INTO trades (market, trade_date_utc, trade_time_utc, trade_price,
            trade_volume, sequential_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (sequential_id) DO NOTHING
        "#,
    )
    .bind(market)
    .bind(trade_date)
    .bind(trade_time)
    .bind(trade_price)
    .bind(trade_volume)
    .bind(sequential_id)
    .execute(pool)
    .await
    .map_err(|e| {
        error!(market = market, error = %e, "Failed to insert trade");
        e
    })?;

    Ok(())
}

/// Handle ticker WebSocket message
/// WebSocket ticker does NOT provide trade_date_kst / trade_time_kst (only REST does)
async fn handle_ticker_msg(
    pool: &PgPool,
    msg: &Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market = msg["code"].as_str().unwrap_or("");
    let trade_date = msg["trade_date"].as_str().unwrap_or("");
    let trade_time = msg["trade_time"].as_str().unwrap_or("");
    let trade_timestamp = msg["trade_timestamp"].as_i64().unwrap_or(0);
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
    let ask_bid = msg["ask_bid"].as_str().unwrap_or("");
    let acc_ask_volume = msg["acc_ask_volume"].as_f64().unwrap_or(0.0);
    let acc_bid_volume = msg["acc_bid_volume"].as_f64().unwrap_or(0.0);

    // WebSocket ticker does NOT provide trade_date_kst / trade_time_kst
    let trade_date_kst = "";
    let trade_time_kst = "";

    sqlx::query(
        r#"
        INSERT INTO tickers (market, trade_date_utc, trade_time_utc, trade_date_kst, trade_time_kst,
            trade_timestamp, opening_price, high_price, low_price, trade_price,
            prev_closing_price, change, change_price, change_rate,
            signed_change_price, signed_change_rate, trade_volume,
            acc_trade_price, acc_trade_price_24h, acc_trade_volume,
            acc_trade_volume_24h, highest_52_week_price, lowest_52_week_price, timestamp,
            ask_bid, acc_ask_volume, acc_bid_volume)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27)
        ON CONFLICT (market, trade_date_utc, trade_time_utc) DO UPDATE SET
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
            timestamp = EXCLUDED.timestamp,
            ask_bid = EXCLUDED.ask_bid,
            acc_ask_volume = EXCLUDED.acc_ask_volume,
            acc_bid_volume = EXCLUDED.acc_bid_volume
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
    .bind(ask_bid)
    .bind(acc_ask_volume)
    .bind(acc_bid_volume)
    .execute(pool)
    .await
    .map_err(|e| {
        error!(market = market, error = %e, "Failed to upsert ticker");
        e
    })?;

    Ok(())
}

/// Handle orderbook WebSocket message
async fn handle_orderbook_msg(
    pool: &PgPool,
    msg: &Value,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let market = msg["code"].as_str().unwrap_or("");
    let timestamp = msg["timestamp"].as_i64().unwrap_or(0);
    let total_ask_size = msg["total_ask_size"].as_f64().unwrap_or(0.0);
    let total_bid_size = msg["total_bid_size"].as_f64().unwrap_or(0.0);
    let orderbook_units = msg["orderbook_units"].clone();

    sqlx::query(
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
    .await
    .map_err(|e| {
        error!(market = market, error = %e, "Failed to insert orderbook");
        e
    })?;

    Ok(())
}
