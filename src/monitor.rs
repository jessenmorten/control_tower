use std::time::Duration;

use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};
use tracing::error;

use crate::{
    config::{self},
    Service, ServiceStatus,
};

pub fn spawn_monitor(tx: UnboundedSender<Service>) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let config = config::get_config();
            for service in config.services {
                let tx = tx.clone();
                tokio::spawn(async move {
                    let mut status: Option<ServiceStatus> = None;

                    if let Some(ping) = service.http_ping {
                        let s = http_ping_service(ping.url.clone(), ping.status_code).await;
                        status = Some(s);
                    }

                    if let Some(status) = status {
                        tx.send(Service {
                            name: service.name,
                            status,
                            dependencies: service.dependencies,
                        })
                        .expect("failed to send service");
                        return;
                    }

                    if let Some(ping) = service.tcp_ping {
                        let s = tcp_ping_service(ping.host.clone(), ping.port).await;
                        status = Some(s);
                    }

                    if let Some(status) = status {
                        tx.send(Service {
                            name: service.name,
                            status,
                            dependencies: service.dependencies,
                        })
                        .expect("failed to send service");
                        return;
                    }
                });
            }

            let duration = Duration::from_secs(config.monitor_interval_seconds);
            tokio::time::sleep(duration).await;
        }
    })
}

async fn http_ping_service(url: String, status_code: u16) -> ServiceStatus {
    let res = reqwest::get(&url).await;
    match res {
        Ok(res) => {
            if res.status().as_u16() == status_code {
                ServiceStatus::Healthy
            } else {
                ServiceStatus::Unhealthy
            }
        }
        Err(err) => {
            error!("http ping failed: {}", err);
            ServiceStatus::Unhealthy
        }
    }
}

async fn tcp_ping_service(host: String, port: u16) -> ServiceStatus {
    let res = tokio::net::TcpStream::connect((host.as_str(), port)).await;
    match res {
        Ok(_) => ServiceStatus::Healthy,
        Err(err) => {
            error!("tcp ping failed: {}", err);
            ServiceStatus::Unhealthy
        }
    }
}
