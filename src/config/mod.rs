use {
    crate::{config::server::Config as ServerConfig, error, storage::mongo::Config as MongoConfig},
    serde::{de::DeserializeOwned, Deserialize},
};

mod server;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub mongo: MongoConfig,
}

impl Config {
    pub fn from_env() -> Result<Config, error::KeyserverError> {
        Ok(Self {
            server: from_env("KEYSERVER_")?,
            mongo: from_env("KEYSERVER_STORAGE_MONGO_")?,
        })
    }
}

fn from_env<T: DeserializeOwned>(prefix: &str) -> Result<T, envy::Error> {
    envy::prefixed(prefix).from_env()
}
