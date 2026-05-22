mod api;
mod config;
mod db;
mod error;

use clap::Parser;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    #[cfg(feature = "dev")]
    dotenvy::dotenv().ok();

    let cli = config::Cli::parse();
    info!("DATABASE_URL: {}", cli.database_url);

    if !cli.access_key.is_empty() {
        info!("UPBIT_ACCESS_KEY: set");
    }
    if !cli.secret_key.is_empty() {
        info!("UPBIT_SECRET_KEY: set");
    }

    let config_str = std::fs::read_to_string(&cli.config_path)?;
    let api_config: config::ApiConfig = serde_yaml::from_str(&config_str)?;

    info!("REST_URL: {}", api_config.rest_url);
    info!("WS_URL: {}", api_config.ws_url);

    let access_key = if cli.access_key.is_empty() { None } else { Some(cli.access_key.clone()) };
    let secret_key = if cli.secret_key.is_empty() { None } else { Some(cli.secret_key.clone()) };

    let _rest = api::rest::RestClient::new(&api_config.rest_url, access_key.clone(), secret_key.clone());
    info!("REST client created");

    let _ws = api::websocket::WebSocketClient::connect(api_config.ws_url, access_key.clone(), secret_key.clone()).await?;
    info!("WebSocket connected");

    let _pool = db::create_pool(&cli.database_url).await?;
    info!("Database connected");

    tokio::signal::ctrl_c().await?;
    info!("Shutdown");
    Ok(())
}
