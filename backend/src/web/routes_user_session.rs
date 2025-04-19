use axum::{
    body::Body,
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Extension, Json, Router,
};
use bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;

use crate::{
    auth::jwt::Claims,
    middlewares::auth::{
        lookup_user::lookup_user_from_token, session_owner_guard::session_owner_guard,
        verify_jwt::get_claims_from_auth_token,
    },
    models::session::{
        create_session, get_sessions_for_user_id, update_session, NewSessionDTO, ScoringType, SessionError, SessionUpdateDTO
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct SessionSearchPayload {
    scoring_type: Option<ScoringType>,
}

#[derive(thiserror::Error, Debug)]
pub enum SessionWebError {
    #[error("Cannot save session to different user")]
    Unauthorized(String, String),
    #[error("Bson error")]
    BsonError(#[from] bson::oid::Error),
    #[error("Data error")]
    UnexpectedError(#[from] SessionError),
}

impl IntoResponse for SessionWebError {
    fn into_response(self) -> Response<Body> {
        match self {
            SessionWebError::Unauthorized(current_user, attempted_owner) => {
                tracing::error!(
                    "Current user {} tried to save session to user {}",
                    current_user,
                    attempted_owner
                );
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Json(json!({ "error": "Unauthorized" })).to_string().into())
                    .unwrap()
            }
            SessionWebError::UnexpectedError(e) => {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Json(json!({ "error": e.to_string() })).to_string().into())
                    .unwrap()
            }
            SessionWebError::BsonError(e) => {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Json(json!({ "error": e.to_string() })).to_string().into())
                    .unwrap()
            }
        }
    }
}

pub fn routes(state: &AppState) -> Router<AppState> {
    let get_claims_layer =
        middleware::from_fn_with_state(state.clone(), get_claims_from_auth_token);
    let lookup_user_layer = middleware::from_fn_with_state(state.clone(), lookup_user_from_token);
    let session_owner_guard_layer = middleware::from_fn(session_owner_guard);
    Router::<AppState>::new()
        .route("/api/user/{user_id}/sessions", get(session_search))
        .route("/api/user/{user_id}/session", post(create_session_handler))
        .route_layer(session_owner_guard_layer)
        .route("/api/user/{user_id}/session/{session_id}", put(update_session_handler))
        .route_layer(lookup_user_layer)
        .route_layer(get_claims_layer)
}

#[tracing::instrument(skip(db))]
#[debug_handler]
async fn session_search(
    Path(user_id): Path<String>,
    State(AppState {
        mongodb_client: db,
        keys: _,
    }): State<AppState>,
    Json(payload): Json<SessionSearchPayload>,
) -> Result<Json<Value>, SessionWebError> {
    let uid = ObjectId::from_str(&user_id)?;
    let result = get_sessions_for_user_id(&db, &uid, payload.scoring_type).await?;
    Ok(Json(json!(result)))
}

#[tracing::instrument(skip(db))]
#[debug_handler]
async fn create_session_handler(
    Path(user_id): Path<String>,
    Extension(claims): Extension<Claims>,
    State(AppState {
        mongodb_client: db,
        keys: _,
    }): State<AppState>,
    Json(payload): Json<NewSessionDTO>,
) -> Result<Json<Value>, SessionWebError> {
    let owner_id = &payload.owner;
    if &user_id != owner_id || &claims.id != owner_id {
        return Err(SessionWebError::Unauthorized(claims.id, owner_id.clone()));
    }
    let result = create_session(&db, payload).await?;
    Ok(Json(json!(result)))
}

#[tracing::instrument(skip(db))]
#[debug_handler]
async fn update_session_handler(
    Path((user_id, session_id)): Path<(String, String)>,
    Extension(claims): Extension<Claims>,
    State(AppState {
        mongodb_client: db,
        keys: _,
    }): State<AppState>,
    Json(payload): Json<SessionUpdateDTO>,
) -> impl IntoResponse {
    if user_id != claims.id  {
        return SessionWebError::Unauthorized(claims.id, user_id.clone()).into_response();
    }
    match update_session(&db, &session_id, payload).await.map_err(SessionWebError::UnexpectedError)
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => e.into_response(),
    
    }
    // let retval = json!({"updated": session_id});
    // StatusCode::NO_CONTENT.into_response()
        //.body(Json(retval).to_string().into())
        //.unwrap()
}