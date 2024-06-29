use axum::{extract::State, middleware, routing::post, Json, Router};
use serde_json::{json, Value};
use crate::{auth::login::LoginError,middlewares::auth::{lookup_user::lookup_user_from_token, verify_jwt::get_claims_from_auth_token}, models::user::find_user, state::AppState};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct UserSearchPayload {
    username: Option<String>,
    email: Option<String>,
    user_id: Option<String>,
}

pub fn routes(state: &AppState) -> Router<AppState> {
    let get_claims_layer = middleware::from_fn_with_state(state.clone(), get_claims_from_auth_token);
    let lookup_user_layer = middleware::from_fn_with_state(state.clone(), lookup_user_from_token);
    Router::new()
        .route("/api/user/search", post(user_search))
         .route_layer(lookup_user_layer)
         .route_layer(get_claims_layer)
        
}
//find_user(db: &Client, user_id: Option<&str>, username: Option<&str>, email: Option<&str>, salt: Option<&str>)
#[tracing::instrument(skip(db))]
async fn user_search(
    State(AppState{mongodb_client: db, keys: _}): State<AppState>,
    payload: Json<UserSearchPayload>,
) -> Result<Json<Value>, LoginError> {
    let result = find_user(&db, payload.user_id.as_deref(), payload.username.as_deref(), payload.email.as_deref(), None).await?;
    Ok(Json(json!(result)))
}