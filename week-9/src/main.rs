use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use tower_http::{trace::TraceLayer};

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    // note. JSON 포맷 로그
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/json", post(json_response))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn json_response(Json(payload): Json<SamplePayload>) -> (StatusCode, Json<SampleResponse>) {
    let response = SampleResponse { response_name: format!("Hello {}", payload.name) };
    return (StatusCode::OK, Json(response))
}

#[derive(Deserialize)]
struct SamplePayload {
    name: String
}

#[derive(Serialize)]
struct SampleResponse {
    response_name: String
}