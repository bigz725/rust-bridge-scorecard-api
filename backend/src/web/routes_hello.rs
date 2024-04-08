use axum::{middleware, routing::get, Json, Router};
use serde_json::{json, Value};
use tracing::{info, instrument};

use crate::{
    middlewares::{lookup_user::lookup_user_from_token, verify_jwt::get_claims_from_auth_token},
    state::AppState,
};

#[instrument(name = "HelloWorldHandler")]
async fn hello_world_handler() -> Json<Value> {
    info!("Hello_world_handler");
    Json(json!({"message": "Hello, World!"}))
}
#[instrument(name = "ProtectedHelloWorldHandler")]
async fn protected_hello_world_handler() -> Json<Value> {
    info!("Protected_hello_world_handler");
    Json(json!({"message": "Hello, World!  You are authenticated!"}))
}

pub fn routes(state: &AppState) -> Router<AppState> {
    let get_claims_layer = middleware::from_fn_with_state(state.clone(), get_claims_from_auth_token);
    let lookup_user_layer = middleware::from_fn_with_state(state.clone(), lookup_user_from_token);
    Router::new()
        .route("/api/protected", get(protected_hello_world_handler))
        .route_layer(lookup_user_layer)
        .route_layer(get_claims_layer)
        .route("/", get(hello_world_handler))
}
