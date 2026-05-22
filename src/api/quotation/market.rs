use serde_json::Value;
use tracing::{error, info};
use sqlx::PgPool;
use crate::api::rest::RestClient;

pub async fn fetch_and_upsert_markets(pool: &PgPool, rest: &RestClient) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let resp = rest.get("/v1/markets", &[]).await?;
    let parsed: Value = serde_json::from_str(&resp)?;
    let markets = parsed["market_group"]["markets"].as_array()
        .ok_or("markets not an array")?;

    info!("Fetched {} markets from Upbit API", markets.len());

    for market in markets {
        let market_code = market["market"].as_str().unwrap_or("");
        let korean_name = market["korean_name"].as_str().unwrap_or("");
        let english_name = market["english_name"].as_str().unwrap_or("");
        let market_warning = match market["market_warning"].as_str() {
            Some("WARN") => true,
            _ => false,
        };
        let caution_price = market["market_event_caution_price_fluctuations"].as_bool().unwrap_or(false);
        let caution_volume = market["market_event_caution_trading_volume_soloing"].as_bool().unwrap_or(false);
        let caution_deposit = market["market_event_caution_deposit_amount_soloing"].as_bool().unwrap_or(false);
        let caution_global = market["market_event_caution_global_price_differences"].as_bool().unwrap_or(false);
        let caution_concentration = market["market_event_caution_concentration_of_small_accounts"].as_bool().unwrap_or(false);

        if let Err(e) = upsert_market(pool, market_code, korean_name, english_name,
            market_warning, caution_price, caution_volume, caution_deposit, caution_global, caution_concentration).await {
            error!(market = market_code, error = %e, "Failed to upsert market");
        }
    }

    info!("Markets upsert completed");
    Ok(())
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
