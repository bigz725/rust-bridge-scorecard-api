use axum::{routing::get, Json, Router};
use serde_json::{Value, json};

use crate::AppState;

async fn hello_world_handler() -> Json<Value> {
    Json(json!({"message": "Hello, World!"}))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_world_handler))

}