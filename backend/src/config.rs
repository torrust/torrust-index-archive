use std::fs;
use config::{ConfigError, Config, File};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Website {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracker {
    pub url: String,
    pub api_url: String,
    pub token: String,
    pub token_valid_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub port: u16,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    pub min_password_length: usize,
    pub max_password_length: usize,
    pub secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub connect_url: String,
    pub torrent_info_update_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub upload_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mail {
    pub email_verification_enabled: bool,
    pub from: String,
    pub reply_to: String,
    pub username: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrustConfig {
    pub website: Website,
    pub tracker: Tracker,
    pub net: Network,
    pub auth: Auth,
    pub database: Database,
    pub storage: Storage,
    pub mail: Mail,
}

impl TorrustConfig {
    pub fn default() -> TorrustConfig {
        TorrustConfig {
            website: Website {
                name: "Torrust".to_string()
            },
            tracker: Tracker {
                url: "udp://localhost:6969".to_string(),
                api_url: "http://localhost:1212".to_string(),
                token: "MyAccessToken".to_string(),
                token_valid_seconds: 7257600
            },
            net: Network {
                port: 3000,
                base_url: None
            },
            auth: Auth {
                min_password_length: 6,
                max_password_length: 64,
                secret_key: "MaxVerstappenWC2021".to_string()
            },
            database: Database {
                connect_url: "sqlite://data.db?mode=rwc".to_string(),
                torrent_info_update_interval: 3600
            },
            storage: Storage {
                upload_path: "./uploads".to_string()
            },
            mail: Mail {
                email_verification_enabled: false,
                from: "example@email.com".to_string(),
                reply_to: "noreply@email.com".to_string(),
                username: "".to_string(),
                password: "".to_string(),
                server: "".to_string(),
                port: 25
            }
        }
    }

    pub fn load_from_file() -> Result<TorrustConfig, ConfigError> {
        let mut config = Config::new();

        const CONFIG_PATH: &str = "config.toml";

        if Path::new(CONFIG_PATH).exists() {
            config.merge(File::with_name(CONFIG_PATH))?;
        } else {
            eprintln!("No config file found.");
            eprintln!("Creating config file..");
            let config = TorrustConfig::default();
            let _ = config.save_to_file();
            return Err(ConfigError::Message(format!("Please edit the config.TOML in the root folder and restart the tracker.")))
        }

        match config.try_into() {
            Ok(data) => Ok(data),
            Err(e) => Err(ConfigError::Message(format!("Errors while processing config: {}.", e))),
        }
    }

    pub fn save_to_file(&self) -> Result<(), ()>{
        let toml_string = toml::to_string(self).expect("Could not encode TOML value");
        fs::write("config.toml", toml_string).expect("Could not write to file!");
        Ok(())
    }
}
