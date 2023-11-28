
pub use self::error::{Error, Result};
pub use self::state::AppState;
use std::net::SocketAddr;

use axum::handler::HandlerWithoutStateExt;
use axum::{response::Json, routing::get, Router,};
use mongodb::{Client, options::ServerAddress};
use serde_json::{json, Value};
use tokio::net::TcpListener;


mod error;
mod web;
mod state;

async fn hello_world_handler() -> Json<Value> {
    Json(json!({"message": "Hello, World!"}))
}

#[tokio::main]
async fn main () {
    println!("Starting server...");
    dotenv::dotenv().ok();
    println!("Loaded .env file.");


    let addr = bind_addr();
    let listener = listener(addr).await;

    let state = AppState {
        mongodb_client: db_conn()
    };

    println!("Connected to MongoDB.");

    
    let app = Router::new()
    .route("/", get(hello_world_handler))
    .merge(web::routes_login::routes())
    .with_state(state).into_make_service();
    axum::serve(listener, app).await.unwrap();

    println!("Listening on {}", addr);
}

async fn listener(addr: SocketAddr) -> TcpListener {
    TcpListener::bind(addr).await.expect("Failed to bind to port")
}

fn bind_addr() -> SocketAddr {
    let port = dotenv::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port_int = port.parse::<u16>().unwrap();
    SocketAddr::from(([0,0,0,0], port_int))
}

fn db_conn() -> Client {
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

