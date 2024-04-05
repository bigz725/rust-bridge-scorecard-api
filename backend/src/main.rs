mod auth;
mod config;
mod error;
mod middlewares;
mod models;
mod state;
mod web;

pub use self::error::{Error, Result};
pub use self::state::AppState;

pub use config::db::db_conn_simple;
pub use config::logging::init_logging;
use config::networking::{bind_addr, listener};

use axum::Router;
use config::logging::env_level;
use tower_http::trace::{self, TraceLayer};

use tracing::info;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_logging();
    info!("Starting server...");

    let level = env_level();

    let addr = bind_addr();
    let listener = listener(config::networking::bind_addr()).await;

    let state = AppState {
        mongodb_client: db_conn_simple().await,
    };
    info!("Connected to MongoDB.");

    let app = Router::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(level))
                .on_response(trace::DefaultOnResponse::new().level(level)),
        )
        .merge(web::routes_hello::routes(&state))
        .merge(web::routes_login::routes())
        .with_state(state)
        .into_make_service();
    info!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
