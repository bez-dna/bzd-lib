use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Deserialize, Clone)]
pub struct HttpSettings {
    pub endpoint: String,
}

#[derive(Deserialize, Clone)]
pub struct DBSettings {
    pub endpoint: String,
}

#[derive(Deserialize, Clone)]
pub struct NATSSettings {
    pub endpoint: String,
    pub stream: String,
}

#[derive(Deserialize, Clone)]
pub struct NATSConsumerSettings {
    pub subjects: Vec<String>,
    pub consumer: String,
}

pub trait Settings<T: DeserializeOwned> {
    fn new() -> Result<T, ConfigError> {
        let app_dir = env::var("APP__DIR").unwrap_or_else(|_| "./settings".into());

        let config = Config::builder()
            .add_source(File::with_name(&format!("{}/default", app_dir)))
            .add_source(File::with_name(&format!("{}/local", app_dir)).required(false))
            .add_source(Environment::with_prefix("app").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
