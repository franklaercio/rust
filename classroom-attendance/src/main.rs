use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler_get))
        .route("/", post(handler_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server starting at http://{:?}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler_get() -> (StatusCode, Json<Value>) {
    let response = json!({
        "greetings": "Hello from GET!"
    });

    (StatusCode::OK, Json(response))
}

async fn handler_post() -> (StatusCode, Json<Value>) {
    let response = json!({
        "greetings": "Hello from POST!"
    });

    (StatusCode::OK, Json(response))
}
