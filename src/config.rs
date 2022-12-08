use {crate::error, serde::Deserialize, std::str::FromStr};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Configuration {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub database_url: String,

    // TELEMETRY
    pub telemetry_enabled: Option<bool>,
    pub telemetry_grpc_url: Option<String>,
}

impl Configuration {
    pub fn new() -> error::Result<Configuration> {
        let config = envy::from_env::<Configuration>()?;
        Ok(config)
    }

    pub fn log_level(&self) -> tracing::Level {
        tracing::Level::from_str(self.log_level.as_str()).expect("Invalid log level")
    }
}

fn default_port() -> u16 {
    3001
}

fn default_log_level() -> String {
    "WARN".to_string()
}
