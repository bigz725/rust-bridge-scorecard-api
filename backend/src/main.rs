mod error;
mod web;
mod state;
mod models;
mod auth;
mod db;
mod networking;

pub use self::error::{Error, Result};
pub use self::state::AppState;
pub use db::db_conn_simple;
use networking::{listener, bind_addr};

use axum::Router;


#[tokio::main]
async fn main () {
    println!("Starting server...");
    dotenv::dotenv().ok();
    println!("Loaded .env file.");


    let addr = bind_addr();
    let listener = listener(networking::bind_addr()).await;

    let state = AppState {
        mongodb_client: db_conn_simple().await
    };
    println!("Connected to MongoDB.");

    println!("Listening on {}", addr);
    let app = Router::new()
        .merge(web::routes_hello::routes())
        .merge(web::routes_login::routes())
        .with_state(state).into_make_service();
    axum::serve(listener, app).await.unwrap();

    println!("Listening on {}", addr);
}





