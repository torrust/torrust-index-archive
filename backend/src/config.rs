use config::{ConfigError, Config, File};
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Tracker {
    pub url: String,
    pub api_url: String,
    pub token: String,
    pub token_valid_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Network {
    pub port: u16,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Auth {
    pub min_password_length: usize,
    pub max_password_length: usize,
    pub secret_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub connect_url: String,
    pub torrent_info_update_interval: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Storage {
    pub upload_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mail {
    pub from: String,
    pub reply_to: String,
    pub username: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TorrustConfig {
    pub tracker: Tracker,
    pub net: Network,
    pub auth: Auth,
    pub database: Database,
    pub storage: Storage,
    pub mail: Mail,
}

impl TorrustConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();

        const CONFIG_PATH: &str = "./config.toml";

        if Path::new(CONFIG_PATH).exists() {
            config.merge(File::with_name(CONFIG_PATH))?;
        }

        match config.try_into() {
            Ok(data) => Ok(data),
            Err(e) => Err(ConfigError::Message(format!("Errors while processing config: {}.", e))),
        }
    }
}
