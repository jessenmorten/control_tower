use serde::Deserialize;

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
    let config = ControlTowerConfig {
        monitor_interval_seconds: 120,

        services: vec![
            ServiceConfig {
                name: "example.com".to_string(),
                http_ping: Some(HttpPingConfig {
                    url: "https://example.com".to_string(),
                    status_code: 200,
                }),
                tcp_ping: None,
            },
            ServiceConfig {
                name: "control_tower".to_string(),
                http_ping: None,
                tcp_ping: Some(TcpPingConfig {
                    host: "127.0.0.1".to_string(),
                    port: 3000,
                }),
            },
        ],
    };

    config
}
