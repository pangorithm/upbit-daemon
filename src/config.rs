use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

/// CLI argument parser
#[derive(Parser)]
#[command(name = "upbit-daemon", about = "업비트 API 정보 수집 데몬")]
pub struct Cli {
    /// PostgreSQL connection URL
    #[arg(env = "DATABASE_URL", required = true)]
    pub database_url: String,
    /// Upbit API access key
    #[arg(env = "UPBIT_ACCESS_KEY", default_value = "")]
    pub access_key: String,
    /// Upbit API secret key
    #[arg(env = "UPBIT_SECRET_KEY", default_value = "")]
    pub secret_key: String,
    /// Path to config.yaml
    #[arg(short, long, default_value = "config.yaml")]
    pub config_path: PathBuf,
}

/// Top-level config: groups URL, candle, rate_limit, partition settings
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    pub url: UrlConfig,
    pub candle: CandleConfig,
    pub subscribe: SubscribeConfig,
    pub rate_limit: RateLimitConfig,
    pub partition: PartitionConfig,
    pub cron: CronConfig,
}

/// Upbit API endpoint URLs
#[derive(Debug, Deserialize, Clone, Default)]
pub struct UrlConfig {
    #[serde(default = "default_rest_url")]
    pub rest: String,
    #[serde(default = "default_ws_url")]
    pub ws: String,
}

/// Candle settings: time units and batch count
#[derive(Debug, Deserialize, Clone, Default)]
pub struct CandleConfig {
    #[serde(default)]
    pub market_prefix: Option<String>,
    #[serde(default = "default_candle_units")]
    pub units: Vec<String>,
    #[serde(default = "default_count")]
    pub count: u32,
}

/// WebSocket subscription settings
#[derive(Debug, Deserialize, Clone, Default)]
pub struct SubscribeConfig {
    #[serde(default)]
    pub candle: Vec<String>,
    #[serde(default)]
    pub ticker: Vec<String>,
    #[serde(default)]
    pub trade: Vec<String>,
    #[serde(default)]
    pub orderbook: Vec<String>,
}

/// Rate limiting settings
#[derive(Debug, Deserialize, Clone, Default)]
pub struct RateLimitConfig {
    #[serde(default = "default_api_calls_per_second")]
    pub api_calls_per_second: usize,
}

/// Partition retention and creation settings
#[derive(Debug, Deserialize, Clone, Default)]
pub struct PartitionConfig {
    #[serde(default = "default_retain_days")]
    pub retain_days: u32,
    #[serde(default = "default_retain_months")]
    pub retain_months: u32,
    #[serde(default = "default_create")]
    pub create: u32,
}

/// Cron schedule settings
#[derive(Debug, Deserialize, Clone, Default)]
pub struct CronConfig {
    #[serde(default)]
    pub candle: Option<String>,
    #[serde(default)]
    pub market: Option<String>,
    #[serde(default)]
    pub subscribe: Option<String>,
}

fn default_rest_url() -> String { "https://api.upbit.com".to_string() }
fn default_ws_url() -> String { "wss://api.upbit.com/websocket/v1".to_string() }
fn default_candle_units() -> Vec<String> { vec!["1m".to_string(), "10m".to_string(), "60m".to_string(), "1d".to_string()] }
fn default_count() -> u32 { 200 }
fn default_api_calls_per_second() -> usize { 5 }
fn default_retain_days() -> u32 { 30 }
fn default_retain_months() -> u32 { 6 }
fn default_create() -> u32 { 3 }
