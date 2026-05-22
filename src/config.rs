use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

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

#[derive(Debug, Deserialize, Default)]
pub struct ApiConfig {
    #[serde(default = "default_rest_url")]
    pub rest_url: String,
    #[serde(default = "default_ws_url")]
    pub ws_url: String,
    #[serde(default = "default_candle_unit")]
    pub candle_unit: u32,
    #[serde(default = "default_batch_size")]
    pub batch_size: u32,
    #[serde(default = "default_api_calls_per_second")]
    pub api_calls_per_second: usize,
    #[serde(default = "default_partition_retain_days")]
    pub partition_retain_days: u32,
    #[serde(default = "default_partition_create_months")]
    pub partition_create_months: u32,
}

fn default_rest_url() -> String { "https://api.upbit.com".to_string() }
fn default_ws_url() -> String { "wss://api.upbit.com/websocket/v1".to_string() }
fn default_candle_unit() -> u32 { 10 }
fn default_batch_size() -> u32 { 200 }
fn default_api_calls_per_second() -> usize { 5 }
fn default_partition_retain_days() -> u32 { 30 }
fn default_partition_create_months() -> u32 { 3 }
