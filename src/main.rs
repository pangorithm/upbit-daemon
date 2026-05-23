mod api;
mod auth;
mod collector;
mod config;
mod cron;
mod db;
mod error;
mod ws;

use clap::Parser;
use tracing::{error, info};

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
}

#[derive(Parser)]
#[command(name = "upbit-daemon", about = "업비트 API 정보 수집 데몬")]
struct Cli {
    #[arg(env = "DATABASE_URL", required = true)]
    database_url: String,
    #[arg(env = "UPBIT_ACCESS_KEY", default_value = "")]
    access_key: String,
    #[arg(env = "UPBIT_SECRET_KEY", default_value = "")]
    secret_key: String,
    #[arg(short, long, default_value = "config.yaml")]
    config_path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();

    #[cfg(feature = "dev")]
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let config_str = std::fs::read_to_string(&cli.config_path)?;
    let config: config::Config = serde_yaml::from_str(&config_str)?;

    info!("REST_URL: {}", config.url.rest);
    info!("WS_URL: {}", config.url.ws);
    info!("market_prefix: {:?}", config.candle.market_prefix);
    info!("candle_units: {:?}", config.candle.units);
    info!("candle_count: {}", config.candle.count);
    info!("rate_limit: {} calls/sec", config.rate_limit.api_calls_per_second);
    info!("partition: create={}, retain_days={}, retain_months={}",
        config.partition.create, config.partition.retain_days, config.partition.retain_months);
    if let Some(ref cron_expr) = config.cron.candle {
        info!("cron.candle: {}", cron_expr);
    }
    if let Some(ref cron_expr) = config.cron.market {
        info!("cron.market: {}", cron_expr);
    }
    if let Some(ref cron_expr) = config.cron.subscribe {
        info!("cron.subscribe: {}", cron_expr);
    }
    if let Some(ref cron_expr) = config.cron.partition {
        info!("cron.partition: {}", cron_expr);
    }

    let access_key = if cli.access_key.is_empty() { None } else { Some(cli.access_key.clone()) };
    let secret_key = if cli.secret_key.is_empty() { None } else { Some(cli.secret_key.clone()) };

    let pool = db::create_pool(&cli.database_url).await?;
    info!("Database connected");

    let rest = api::rest::RestClient::new(&config.url.rest, access_key.clone(), secret_key.clone());
    info!("REST client created");

    if let Err(e) = db::init::init_database(&pool).await {
        error!("Database initialization failed: {}", e);
    }
    if let Err(e) = db::partition::create_future_partitions(&pool, &config).await {
        error!("Failed to create future partitions: {}", e);
    }
    if let Err(e) = api::quotation::market::fetch_and_upsert_markets(&pool, &rest, &config.candle.market_prefix).await {
        error!("Failed to fetch markets: {}", e);
    }

    let ws_rest = rest.clone();
    let ws_rest_market = ws_rest.clone();

    let ws_pool = pool.clone();
    let ws_config = config.clone();
    let ws_handle = tokio::spawn(async move {
        let mut reconnect_delay_secs: u64 = 1;
        let max_reconnect_delay_secs: u64 = 30;
        loop {
            ws::session::run_ws_session(ws_pool.clone(), ws_config.clone(),
                access_key.clone(), secret_key.clone(), "WS").await;
            tokio::time::sleep(std::time::Duration::from_secs(reconnect_delay_secs)).await;
            reconnect_delay_secs = (reconnect_delay_secs * 2).min(max_reconnect_delay_secs);
        }
    });

let cron_config = config.clone();
    let cron_pool = pool.clone();
    let cron_pool_inner = cron_pool.clone();
    tokio::spawn(async move {
        cron::partition_schedule::run_partition_schedule(&cron_pool_inner, &cron_config).await;
    });

    let market_config = config.clone();
    let market_pool = cron_pool.clone();
    let market_rest = ws_rest_market.clone();
    tokio::spawn(async move {
        cron::market_refresh::run_market_refresh(&market_pool, &market_rest, &market_config).await;
    });

   let candle_gap_pool = pool.clone();
    let candle_gap_rest = rest.clone();
    let candle_gap_config = config.clone();
    tokio::spawn(async move {
        collector::candles::run_gap_filling(&candle_gap_pool, &candle_gap_rest, &candle_gap_config).await;
    });

    let _ = ws_handle.await;

    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");
    Ok(())
}
