use {serde::Deserialize, serde_piecewise_default::DeserializePiecewiseDefault};

#[derive(DeserializePiecewiseDefault, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub concurrency_limit: usize,
    pub timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            concurrency_limit: 1024,
            timeout: 10,
        }
    }
}
