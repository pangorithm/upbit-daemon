use crate::api::rest::RestClient;
use serde_json::Value;

pub async fn get_candles_minutes(
    rest: &RestClient,
    market: &str,
    unit: &str,
    count: u32,
    to: &str,
) -> Result<Vec<Value>, crate::error::AppError> {
    let api_unit = unit_to_api_value(unit);
    let query = &[
        ("market", market),
        ("count", &count.to_string()),
        ("to", to),
    ];
    let resp = rest
        .get(&format!("/v1/candles/minutes/{api_unit}"), query)
        .await?;
    Ok(serde_json::from_str(&resp)?)
}

pub async fn get_candles_days(
    rest: &RestClient,
    market: &str,
    to: &str,
    count: u32,
) -> Result<Vec<Value>, crate::error::AppError> {
    let query = &[("market", market), ("to", to), ("count", &count.to_string())];
    let resp = rest.get("/v1/candles/days", query).await?;
    let value: Value = serde_json::from_str(&resp)?;
    if let Some(arr) = value.as_array() {
        Ok(arr.clone())
    } else if let Some(obj) = value.as_object() {
        for (_, v) in obj {
            if let Some(arr) = v.as_array() {
                return Ok(arr.clone());
            }
        }
        Ok(vec![])
    } else {
        Ok(vec![])
    }
}

/// Strip 'm' suffix for API request (e.g., "10m" → "10")
pub fn unit_to_api_value(unit: &str) -> &str {
    unit.strip_suffix('m').unwrap_or(unit)
}

/// Return true if unit suffix is 's' (→ candles_seconds)
pub fn is_seconds_unit(unit: &str) -> bool {
    unit.ends_with('s')
}

/// Return true if unit suffix is 'd' (→ candles_days)
pub fn is_days_unit(unit: &str) -> bool {
    unit.ends_with('d')
}
