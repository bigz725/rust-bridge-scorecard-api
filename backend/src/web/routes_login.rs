use crate::{
    auth::login::{login, LoginError, LoginPayload},
    state::AppState,
};
use axum::{debug_handler, extract::{Json, State}, routing::post, Router};
use serde_json::{json, Value};

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/signin", post(handle_login))
}

#[tracing::instrument(skip(db, keys, payload))]
#[debug_handler]
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
