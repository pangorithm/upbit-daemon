use reqwest::Client;
use serde_json::Value;

pub async fn get_candles_minutes(
    rest: &Client,
    market: &str,
    unit: u32,
    count: u32,
    to: &str,
) -> Result<Vec<Value>, reqwest::Error> {
    let url = format!(
        "https://api.upbit.com/v1/candles/minutes/{}?market={}&count={}&to={}",
        unit, market, count, to
    );

    rest.get(&url).send().await?.json().await
}
