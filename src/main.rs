use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new()
        .route("/", get(root))
        .route("/greet", post(greet));

    info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed to start");
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn greet(Json(payload): Json<GreetingRequest>) -> (StatusCode, Json<GreetingResponse>) {
    let user = GreetingResponse {
        name: payload.name.clone(),
        greeting: format!("Hello, {}!", payload.name),
    };

    info!("greeted {} with {}", payload.name, user.greeting);
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct GreetingRequest {
    name: String,
}

#[derive(Serialize)]
struct GreetingResponse {
    name: String,
    greeting: String,
}
