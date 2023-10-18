use axum::{http::StatusCode, routing::get, Json, Router, Server};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::info;

mod config;
mod monitor;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    monitor::spawn_monitor();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new()
        .route("/api/services", get(get_services))
        .nest_service("/", ServeDir::new("static/"));

    info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed to start");
}

async fn get_services() -> (StatusCode, Json<Vec<Service>>) {
    let now = std::time::Instant::now();
    let flaky_status = if now.elapsed().as_nanos() % 2 == 0 {
        ServiceStatus::Healthy
    } else {
        ServiceStatus::Unhealthy
    };

    let services = vec![
        Service {
            name: "users".to_string(),
            status: ServiceStatus::Healthy,
        },
        Service {
            name: "payments".to_string(),
            status: flaky_status,
        },
        Service {
            name: "products".to_string(),
            status: ServiceStatus::Healthy,
        },
    ];

    (StatusCode::OK, Json(services))
}

#[derive(Serialize, Debug)]
enum ServiceStatus {
    Healthy,
    Unhealthy,
}

#[derive(Serialize)]
struct Service {
    name: String,
    status: ServiceStatus,
}
