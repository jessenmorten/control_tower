use serde::Deserialize;

#[derive(Deserialize)]
pub struct ControlTowerConfig {
    pub monitor_interval_seconds: u64,
    pub services: Vec<ServiceConfig>,
}

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub ping: Option<HttpPingConfig>,
}

#[derive(Deserialize)]
pub struct HttpPingConfig {
    pub url: String,
    pub status_code: u16,
}

pub fn get_config() -> ControlTowerConfig {
    let config = ControlTowerConfig {
        monitor_interval_seconds: 120,
        services: vec![
            ServiceConfig {
                name: "example.com".to_string(),
                ping: Some(HttpPingConfig {
                    url: "https://example.com".to_string(),
                    status_code: 200,
                }),
            },
        ],
    };

    config
}
