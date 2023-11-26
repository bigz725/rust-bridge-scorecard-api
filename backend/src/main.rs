use std::net::SocketAddr;

use axum::{response::Json, routing::get, Router,};
use serde_json::{json, Value};

async fn hello_world_handler() -> Json<Value> {
    Json(json!({"message": "Hello, World!"}))
}

#[tokio::main]
async fn main () {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app: Router<> = 
        Router::new().route("/", get(hello_world_handler));

        axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

