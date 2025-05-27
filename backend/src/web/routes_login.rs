use crate::{
    auth::login::{login, LoginError, LoginPayload},
    state::AppState,
};
use axum::{debug_handler, extract::{Json, State}, routing::post, Router};
use serde_json::{json, Value};


pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/signin", post(handle_login))
}

#[tracing::instrument(skip(diesel_conn, keys, payload))]
#[debug_handler]
async fn handle_login(
    State(AppState {
        db_conn: _,
        diesel_conn,
        keys,
    }): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>, LoginError> {
    let result = login(&diesel_conn, &keys, payload).await?;
    Ok(Json(json!(result)))
}
