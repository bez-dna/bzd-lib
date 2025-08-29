use std::env;

use config::{Config, Environment, File};
use serde::{Deserialize, de::DeserializeOwned};

use crate::error::Error;

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

pub trait Settings<T: DeserializeOwned> {
    fn new() -> Result<T, Error> {
        let app_dir = if let Ok(dir) = env::var("APP_DIR") {
            dir
        } else {
            ".".into()
        };

        let config = Config::builder()
            .add_source(File::with_name(&format!("{}/settings/default", app_dir)))
            .add_source(File::with_name(&format!("{}/settings/local", app_dir)).required(false))
            .add_source(Environment::with_prefix("app").separator("_"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
