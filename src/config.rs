use {crate::error, relay_rpc::domain::ProjectId, serde::Deserialize, std::str::FromStr};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Configuration {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub database_url: String,
    pub project_id: ProjectId,

    #[serde(default = "default_blockchain_api_endpoint")]
    pub blockchain_api_endpoint: Option<String>,

    // Telemetry
    pub telemetry_enabled: Option<bool>,
    pub telemetry_grpc_url: Option<String>,
    pub telemetry_prometheus_port: Option<u16>,

    // AWS
    pub s3_endpoint: Option<String>,

    // GeoIP
    pub geoip_db_bucket: Option<String>,
    pub geoip_db_key: Option<String>,

    // GeoBlocking
    pub blocked_countries: Vec<String>,
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
    8080
}

fn default_log_level() -> String {
    "WARN".to_string()
}

fn default_blockchain_api_endpoint() -> Option<String> {
    Some("https://rpc.walletconnect.com".to_string())
}
