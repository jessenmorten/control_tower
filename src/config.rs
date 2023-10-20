use serde::Deserialize;
use serde_json::from_reader;
use std::{fs::File, io::BufReader, path::Path};
use tracing::{error, info};

#[derive(Deserialize)]
pub struct ControlTowerConfig {
    pub monitor_interval_seconds: u64,
    pub services: Vec<ServiceConfig>,
}

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub http_ping: Option<HttpPingConfig>,
    pub tcp_ping: Option<TcpPingConfig>,
}

#[derive(Deserialize)]
pub struct HttpPingConfig {
    pub url: String,
    pub status_code: u16,
}

#[derive(Deserialize)]
pub struct TcpPingConfig {
    pub host: String,
    pub port: u16,
}

pub fn get_config() -> ControlTowerConfig {
    let path = Path::new("config.json");
    let config = match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match from_reader(reader) {
                Ok(config) => config,
                Err(err) => {
                    error!("Failed to parse config.json: {}", err);
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            error!("Failed to open config.json: {}", err);
            std::process::exit(1);
        }
    };

    info!("Loaded config.json");
    config
}
