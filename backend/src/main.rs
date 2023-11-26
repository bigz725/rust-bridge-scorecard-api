use std::net::SocketAddr;

use axum::{response::Json, routing::get, Router,};
use mongodb::{Client, options::ServerAddress};
use serde_json::{json, Value};

async fn hello_world_handler() -> Json<Value> {
    Json(json!({"message": "Hello, World!"}))
}

#[tokio::main]
async fn main () {
    println!("Starting server...");
    dotenv::dotenv().ok();
    println!("Loaded .env file.");

    let db_conn = db_conn().await;
    println!("Connected to MongoDB.");
    let port = dotenv::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port_int = port.parse::<u16>().unwrap();
    let addr = SocketAddr::from(([0,0,0,0], port_int));
    let app = 
        Router::new()
          .route("/", get(hello_world_handler))
          .with_state(db_conn)
          .into_make_service_with_connect_info::<SocketAddr>();

    println!("App created, binding to {}...", addr);
    axum::Server::bind(&addr).serve(app).await.expect("Failed to start server");

    println!("Listening on {}", addr);
    ()
}

async fn db_conn() -> Client {
    let uri = dotenv::var("MONGODB_URL").unwrap_or_else(|_| "mongodb://localhost/bridge_scorecard_api".to_string());
    let server_addresses = vec![ServerAddress::Tcp { host: uri, port: None}];
    let min_pool_size = dotenv::var("MONGODB_MIN_POOL_SIZE").unwrap_or_else(|_| "1".to_string());
    let max_pool_size = dotenv::var("MONGODB_MAX_POOL_SIZE").unwrap_or_else(|_| "10".to_string());
    let (min_pool_size_int, max_pool_size_int) = (
        min_pool_size.parse::<u32>().unwrap(),
        max_pool_size.parse::<u32>().unwrap()
    );
    let options = mongodb::options::ClientOptions::builder()
        .min_pool_size(min_pool_size_int)
        .max_pool_size(max_pool_size_int)
        .hosts(server_addresses)
        .build();

    Client::with_options(options).expect("Failed to initialize MongoDB client.")
}

