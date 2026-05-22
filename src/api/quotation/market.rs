use serde_json::Value;
use tracing::{error, info};
use sqlx::PgPool;
use crate::api::rest::RestClient;

pub async fn fetch_and_upsert_markets(pool: &PgPool, rest: &RestClient) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let resp = rest.get("/v1/markets", &[]).await?;
    let parsed: Value = serde_json::from_str(&resp)?;

    // /v1/markets returns a flat array
    let markets: Vec<Value> = match parsed.as_array() {
        Some(arr) => arr.clone(),
        None => {
            error!("Upbit /v1/markets returned non-array response: {}", resp);
            return Ok(());
        }
    };

    let filtered: Vec<&Value> = markets.iter()
        .filter(|m| {
            let market = m["market"].as_str().unwrap_or("");
            !market.is_empty() && !market.contains(":") // Exclude derivative markets
        })
        .collect();

    info!("Fetched {} markets from Upbit API", filtered.len());

    for market in &filtered {
        let market_code = market["market"].as_str().unwrap_or("");
        let korean_name = market["korean_name"].as_str().unwrap_or("");
        let english_name = market["english_name"].as_str().unwrap_or("");

        // Parse nested market_event structure from /v1/market/all response
        let (market_warning, caution_price, caution_volume, caution_deposit, caution_global, caution_concentration) =
            parse_market_events(market);

        if let Err(e) = upsert_market(pool, market_code, korean_name, english_name,
            market_warning, caution_price, caution_volume, caution_deposit, caution_global, caution_concentration).await {
            error!(market = market_code, error = %e, "Failed to upsert market");
        }
    }

    info!("Markets upsert completed");
    Ok(())
}

fn parse_market_events(market: &Value) -> (bool, bool, bool, bool, bool, bool) {
    // Check both top-level and nested market_event structures
    if let Some(events) = market.get("market_event") {
        let warning = events["warning"].as_bool().unwrap_or(false);
        let caution_obj = events.get("caution").and_then(|c| c.as_object()).cloned();
        let (price_fluctuations, trading_volume, deposit_amount, global_price, concentration) = match caution_obj {
            Some(ref c) => (
                c["PRICE_FLUCTUATIONS"].as_bool().unwrap_or(false),
                c["TRADING_VOLUME_SOARING"].as_bool().unwrap_or(false),
                c["DEPOSIT_AMOUNT_SOARING"].as_bool().unwrap_or(false),
                c["GLOBAL_PRICE_DIFFERENCES"].as_bool().unwrap_or(false),
                c["CONCENTRATION_OF_SMALL_ACCOUNTS"].as_bool().unwrap_or(false),
            ),
            None => (false, false, false, false, false),
        };
        return (warning, price_fluctuations, trading_volume, deposit_amount, global_price, concentration);
    }

    // Legacy flat structure
    let warning = market["market_warning"].as_str() == Some("WARN");
    let price_fluctuations = market["market_event_caution_price_fluctuations"].as_bool().unwrap_or(false);
    let trading_volume = market["market_event_caution_trading_volume_soloing"].as_bool().unwrap_or(false);
    let deposit_amount = market["market_event_caution_deposit_amount_soloing"].as_bool().unwrap_or(false);
    let global_price = market["market_event_caution_global_price_differences"].as_bool().unwrap_or(false);
    let concentration = market["market_event_caution_concentration_of_small_accounts"].as_bool().unwrap_or(false);
    (warning, price_fluctuations, trading_volume, deposit_amount, global_price, concentration)
}

async fn upsert_market(
    pool: &PgPool,
    market: &str,
    korean_name: &str,
    english_name: &str,
    market_warning: bool,
    caution_price: bool,
    caution_volume: bool,
    caution_deposit: bool,
    caution_global: bool,
    caution_concentration: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO markets (market, korean_name, english_name,
            market_event_warning, market_event_caution_price_fluctuations,
            market_event_caution_trading_volume_soloing,
            market_event_caution_deposit_amount_soloing,
            market_event_caution_global_price_differences,
            market_event_caution_concentration_of_small_accounts)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (market) DO UPDATE SET
            korean_name = EXCLUDED.korean_name,
            english_name = EXCLUDED.english_name,
            market_event_warning = EXCLUDED.market_event_warning,
            market_event_caution_price_fluctuations = EXCLUDED.market_event_caution_price_fluctuations,
            market_event_caution_trading_volume_soloing = EXCLUDED.market_event_caution_trading_volume_soloing,
            market_event_caution_deposit_amount_soloing = EXCLUDED.market_event_caution_deposit_amount_soloing,
            market_event_caution_global_price_differences = EXCLUDED.market_event_caution_global_price_differences,
            market_event_caution_concentration_of_small_accounts = EXCLUDED.market_event_caution_concentration_of_small_accounts
        "#,
    )
    .bind(market)
    .bind(korean_name)
    .bind(english_name)
    .bind(market_warning)
    .bind(caution_price)
    .bind(caution_volume)
    .bind(caution_deposit)
    .bind(caution_global)
    .bind(caution_concentration)
    .execute(pool)
    .await?;

    Ok(())
}
