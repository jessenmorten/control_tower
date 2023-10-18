use std::time::Duration;

use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};

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
                    if let Some(ping) = service.ping {
                        let status = ping_service(ping.url.clone(), ping.status_code).await;
                        let service = Service {
                            name: service.name,
                            status,
                        };
                        tx.send(service).expect("failed to send service");
                    }
                });
            }

            let duration = Duration::from_secs(config.monitor_interval_seconds);
            tokio::time::sleep(duration).await;
        }
    })
}

fn ping_service(url: String, status_code: u16) -> impl std::future::Future<Output = ServiceStatus> {
    async move {
        let res = reqwest::get(&url).await;
        match res {
            Ok(res) => {
                if res.status().as_u16() == status_code {
                    ServiceStatus::Healthy
                } else {
                    ServiceStatus::Unhealthy
                }
            }
            Err(_) => ServiceStatus::Unhealthy,
        }
    }
}
