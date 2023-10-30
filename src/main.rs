use axum::{extract::State, http::StatusCode, routing::get, Json, Router, Server};
use serde::Serialize;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tower_http::services::ServeDir;
use tracing::info;

mod config;
mod monitor;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    monitor::spawn_monitor(tx);

    let shared_state = SharedState::default();
    let state = shared_state.clone();

    tokio::spawn(async move {
        while let Some(service) = rx.recv().await {
            state
                .write()
                .unwrap()
                .services
                .insert(service.name.clone(), service.clone());
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new()
        .route("/api/services", get(get_services))
        .nest_service("/", ServeDir::new("static/"))
        .with_state(Arc::clone(&shared_state));

    info!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed to start");
}

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    services: HashMap<String, Service>,
}

async fn get_services(State(state): State<SharedState>) -> (StatusCode, Json<Vec<Service>>) {
    let mut services: Vec<Service> = state.read().unwrap().services.values().cloned().collect();
    services.sort_by(|a, b| a.name.cmp(&b.name));
    (StatusCode::OK, Json(services))
}

#[derive(Serialize, Debug, Clone)]
enum ServiceStatus {
    Healthy,
    Unhealthy,
}

#[derive(Serialize, Debug, Clone)]
pub struct Service {
    name: String,
    status: ServiceStatus,
    dependencies: Vec<String>,
}
