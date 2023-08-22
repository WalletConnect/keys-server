use {
    dotenv::dotenv,
    keyserver::{bootstrap, config::Configuration, error, log::Logger},
    tokio::sync::broadcast,
};

#[tokio::main]
async fn main() -> error::Result<()> {
    let logger = Logger::init().expect("Failed to start logging");
    let (_signal, shutdown) = broadcast::channel(1);
    dotenv().ok();
    let config = Configuration::new().expect("Failed to load config!");
    let result = bootstrap(shutdown, config).await;

    logger.stop();

    result
}
