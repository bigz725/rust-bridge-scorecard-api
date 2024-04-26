use crate::{
    auth::jwt::{create_token, Claims},
    models::user::{find_user_by_username, Role, User, UserError, UserError::InvalidCredentials},
    state::AppState, 
};
use axum::{body::Body, extract::State, response::{IntoResponse, Response}, routing::post, Json, Router};
use bcrypt::verify;
use chrono::Utc;
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::{info, instrument, warn};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String, //TODO: Make this a Secret
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something's gone wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl From<UserError> for LoginError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::BadDecryption(_) | UserError::QueryError(_) | UserError::InvalidUserRecord(_) => {
                LoginError::UnexpectedError(err.into())
            }
            _ => Self::AuthError(err.into())
        }
    }
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response<Body> {
        match self {
            LoginError::AuthError(_) => {
                (axum::http::StatusCode::UNAUTHORIZED, unable_to_login_json()).into_response()
            }
            LoginError::UnexpectedError(_) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, unable_to_login_json()).into_response()
            }
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/signin", post(login))
}

#[instrument(name = "Login", skip(db, keys))]
async fn login(
    State(AppState{mongodb_client: db, keys}): State<AppState>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>, LoginError> {
    let user = find_user_by_username(&db, &payload.username).await?;
    let verify_result =
        verify(&payload.password, &user.password).map_err(UserError::BadDecryption)?;

    if verify_result {
        let claims = create_claims(&user);
        let token = create_token(&claims, &keys.encoding)?;
        info!("User {} successfully logged in", user.username);
        let response = response(user, token);

        Ok(response)
    } else {
        warn!("Password incorrect for user {}", payload.username);
        Err(InvalidCredentials)?
    }
}

fn process_roles(roles: Vec<Role>) -> Vec<String> {
    roles
        .iter()
        .map(|role| format!("ROLE_{}", role.name.to_ascii_uppercase()))
        .collect::<Vec<String>>()
}

fn response(user: User, token: String) -> Json<Value> {
    let doc = json!({
        "id": user.id.to_string(),
        "username": user.username,
        "email": user.email,
        "roles": process_roles(user.roles),
        "accessToken": token,
    });
    Json(doc)
}

fn create_claims(user: &User) -> Claims {
    let now = Utc::now().timestamp();
    Claims {
        id: user.id.to_string(),
        salt: user.salt.clone(),
        exp: add_time(now, 1),
    }
}

fn add_time(from: i64, days: i64) -> usize {
    let sixty_four = from + (days * 86400);
    sixty_four as usize
}

fn unable_to_login_json() -> Json<Value> {
    Json(json!({"message": "Unable to login"}))
}