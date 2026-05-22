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
}

fn default_rest_url() -> String { "https://api.upbit.com".to_string() }
fn default_ws_url() -> String { "wss://api.upbit.com/websocket/v1".to_string() }
