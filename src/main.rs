use env_logger::Builder;
use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval},
    trade
};
use config::{Config, File};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Logs setup
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    // Load API-keys
    let settings = Config::builder()
        .add_source(File::with_name("config/config"))
        .build()
        .unwrap();

    let api_key: String = settings.get("binance.api_key").unwrap();
    let secret_key: String = settings.get("binance.secret_key").unwrap();

    // Binance client init
    let credentials = Credentials::from_hmac(api_key, secret_key);
    let client = BinanceHttpClient::default().credentials(credentials);

    // Get candlesticks for BTCUSDT with 1min interval
    let data = client
        .send(market::klines("BTCUSDT", KlineInterval::Minutes1))
        .await?
        .into_body_str()
        .await?;
    log::info!("{}", data);

    // Get last 10 candlesticks for BTCUSDT with 1h interval
    let data = client
        .send(market::klines("BTCUSDT", KlineInterval::Hours1).limit(10))
        .await?
        .into_body_str()
        .await?;
    log::info!("{}", data);

    // Get account info
    let data = client
        .send(trade::account())
        .await?
        .into_body_str()
        .await?;
    log::info!("{}", data);

    Ok(())
}