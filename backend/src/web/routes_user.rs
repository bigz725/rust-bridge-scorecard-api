use axum::{body::Body, debug_handler, extract::{Path, State}, http::StatusCode, middleware, response::{IntoResponse, Response}, routing::{put,post}, Extension, Json, Router};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::{auth::login::LoginError,middlewares::auth::{lookup_user::lookup_user_from_token, verify_jwt::get_claims_from_auth_token}, models::user::{find_user, update_user, User, UserError}, state::AppState};
use serde::Deserialize;



#[derive(Debug, Deserialize)]
struct UserSearchPayload {
    username: Option<String>,
    email: Option<String>,
    user_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct UserUpdatePayload {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(thiserror::Error, Debug)]
enum UserWebError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Data error")]
    DbError(#[from] UserError),
    // #[error("Unexpected error")]
    // UnexpectedError,
}

impl IntoResponse for UserWebError {
    fn into_response(self) -> Response<Body> {
        match self {
            UserWebError::Unauthorized => {
                tracing::error!("Unauthorized");
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Json(json!({ "error": "Unauthorized" })).to_string().into())
                    .unwrap()
            }
            UserWebError::DbError(err) => {
                tracing::error!("Data error {:?}", err);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Json(json!({ "error": "Data error" })).to_string().into())
                    .unwrap()
            }
            // UserWebError::UnexpectedError => {
            //     tracing::error!("Unexpected error");
            //     Response::builder()
            //         .status(StatusCode::INTERNAL_SERVER_ERROR)
            //         .body(Json(json!({ "error": "Unexpected error" })).to_string().into())
            //         .unwrap()
            // }
        }
    }
}

pub fn routes(state: &AppState) -> Router<AppState> {
    let get_claims_layer = middleware::from_fn_with_state(state.clone(), get_claims_from_auth_token);
    let lookup_user_layer = middleware::from_fn_with_state(state.clone(), lookup_user_from_token);
    Router::new()
        .route("/api/user/search", post(user_search))
        .route("/api/user/:user_id", put(user_update))
        .route_layer(lookup_user_layer)
        .route_layer(get_claims_layer)
        
}

#[tracing::instrument(skip(diesel_conn))]
#[debug_handler]
async fn user_search(
    State(AppState{db_conn: _, diesel_conn, keys: _}): State<AppState>,
    payload: Json<UserSearchPayload>,
) -> Result<Json<Value>, LoginError> {
    let result = find_user(&diesel_conn, payload.user_id.as_deref(), payload.username.as_deref(), payload.email.as_deref(), None).await?;
    Ok(Json(json!(result)))
}

#[tracing::instrument(skip(diesel_conn))]
#[debug_handler]
async fn user_update(
    Path(target_user_id): Path<String>,
    Extension(current_user): Extension<User>,
    State(AppState{db_conn: _, diesel_conn, keys: _}): State<AppState>,
    Json(payload): Json<UserUpdatePayload>,
) -> impl IntoResponse {
    if current_user.id.to_string() != target_user_id {
        return UserWebError::Unauthorized.into_response();
    }
    let result = update_user(&diesel_conn, payload.into())
        .await
        .map_err(UserWebError::DbError);
    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.into_response(),
    }

}
