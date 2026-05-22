use crate::api::rest::RestClient;
use serde_json::Value;

pub async fn get_candles_minutes(
    rest: &RestClient,
    market: &str,
    unit: u32,
    count: u32,
    to: &str,
) -> Result<Vec<Value>, crate::error::AppError> {
    let query = &[
        ("market", market),
        ("count", &count.to_string()),
        ("to", to),
    ];
    let resp = rest.get(&format!("/v1/candles/minutes/{unit}"), query).await?;
    Ok(serde_json::from_str(&resp)?)
}

pub async fn get_candles_days(
    rest: &RestClient,
    market: &str,
    to: &str,
) -> Result<Vec<Value>, crate::error::AppError> {
    let query = &[
        ("market", market),
        ("to", to),
    ];
    let resp = rest.get("/v1/candles/days", query).await?;
    Ok(serde_json::from_str(&resp)?)
}
