pub use config::ConfigError;
use config::{Config, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Database {
    pub connection_string: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: Database,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}
