use config::ConfigError;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: ServiceConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg: Config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();
        Ok(cfg)
    }
}
