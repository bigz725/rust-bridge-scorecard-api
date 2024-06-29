use crate::{
    auth::login::{login, LoginError, LoginPayload},
    state::AppState,
};
use axum::{extract::Json, extract::State, routing::post, Router};
use serde_json::{json, Value};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/signin", post(handle_login))
}

async fn handle_login(
    State(AppState {
        mongodb_client: db,
        keys,
    }): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>, LoginError> {
    let result = login(&db, &keys, payload).await?;
    Ok(Json(json!(result)))
}
