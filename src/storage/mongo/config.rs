use {serde::Deserialize, serde_piecewise_default::DeserializePiecewiseDefault};

#[derive(DeserializePiecewiseDefault, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub addr: Option<String>,
    pub database: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: None,
            database: "keyserver".to_string(),
        }
    }
}
