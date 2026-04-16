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

    // Forward to service-b
    let client = reqwest::Client::new();
    match client
        .post("http://service-b/api/data")
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            let counter = resp.text().await.unwrap_or_default();
            tracing::info!("Service-B response: {:?} {:?}", status, counter);
            (
                StatusCode::CREATED,
                Json(serde_json::json!({"received": true, "numOfRequests": counter, "message:": payload.message})),
            )
        }
        Err(e) => {
            tracing::error!("Failed to reach service-b: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"received": false, "error": e.to_string()})),
            )
        }
    }
    
}
