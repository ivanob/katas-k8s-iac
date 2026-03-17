use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataPayload {
    pub message: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/data", post(receive_data));

    // Run it on 0.0.0.0:8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port 8080");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

// GET /health
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
}

// POST /api/data
async fn receive_data(Json(payload): Json<DataPayload>) -> impl IntoResponse {
    tracing::info!("Received data: {:?}", payload);

    // TODO: Forward to service-b if needed
    // TODO: Process data, store in database, etc.

    (
        StatusCode::CREATED,
        Json(serde_json::json!({"received": true, "message": payload.message})),
    )
}
