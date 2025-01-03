use axum::{
    debug_handler,
    extract::State,
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};
use crate::{models::session::get_sessions, state::AppState};

use super::routes_user_session::SessionWebError;

pub fn routes() -> Router<AppState> {
    // let get_claims_layer =
    // middleware::from_fn_with_state(state.clone(), get_claims_from_auth_token);
    // let lookup_user_layer = middleware::from_fn_with_state(state.clone(), lookup_user_from_token);
    // let session_owner_guard_layer = middleware::from_fn(session_owner_guard);
    Router::<AppState>::new()
        .route("/api/sessions", get(session_search))
}

#[tracing::instrument(skip(db))]
#[debug_handler]
async fn session_search(
    State(AppState {
        db_conn: db,
        diesel_conn: _,
        keys: _,
    }): State<AppState>,
) -> Result<Json<Value>, SessionWebError> {
    let result = get_sessions(&db, None).await?;
    Ok(Json(json!(result)))
}