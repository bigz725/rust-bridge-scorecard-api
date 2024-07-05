use crate::{
    auth::jwt::{create_token, Claims},
    models::user::{
        find_user, Role, User,
        UserError::{self, InvalidCredentials},
    },
};
use async_graphql::{InputObject, SimpleObject};
use axum::{
    body::Body,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::verify;
use chrono::Utc;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::jwt::Keys;
#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something's gone wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct LoginResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub access_token: String,
}

impl LoginResponse {
    pub fn new(user: User, token: String) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            roles: process_roles(user.roles),
            access_token: token,
        }
    }
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response<Body> {
        Json(json!({
            "id": self.id,
            "username": self.username,
            "email": self.email,
            "roles": self.roles,
            "accessToken": self.access_token,
        }))
        .into_response()
    }
}

#[derive(Debug, Deserialize, InputObject)]
pub struct LoginPayload {
    username: String,
    password: String, //TODO: Make this a Secret
}

impl From<UserError> for LoginError {
    fn from(err: UserError) -> Self {
        match err {
            UserError::BadDecryption(_)
            | UserError::QueryError(_)
            | UserError::InvalidUserRecord(_) => LoginError::UnexpectedError(err.into()),
            _ => Self::AuthError(err.into()),
        }
    }
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response<Body> {
        match self {
            LoginError::AuthError(_) => {
                (axum::http::StatusCode::UNAUTHORIZED, unable_to_login_json()).into_response()
            }
            LoginError::UnexpectedError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                unable_to_login_json(),
            )
                .into_response(),
        }
    }
}

#[tracing::instrument(target = "login", skip(db, keys, payload))]
pub async fn login(
    db: &Client,
    keys: &Keys,
    payload: LoginPayload,
) -> Result<LoginResponse, LoginError> {
    
    let users = find_user(db, None, Some(&payload.username), None, None).await?;
    if users.len() > 1 {
        tracing::warn!("Multiple users found with username {}", payload.username);
        Err(UserError::UserNotFound)?
    }
    let user = users.first().ok_or(UserError::UserNotFound)?.to_owned();
    let verify_result =
        verify(&payload.password, &user.password).map_err(UserError::BadDecryption)?;

    if verify_result {
        let claims = create_claims(&user);
        let token = create_token(&claims, &keys.encoding)?;
        tracing::info!("User {} successfully logged in", user.username);
        let response = LoginResponse::new(user, token);
        Ok(response)
    } else {
        tracing::warn!("Password incorrect for user {}", payload.username);
        Err(InvalidCredentials)?
    }
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

pub fn process_roles(roles: Vec<Role>) -> Vec<String> {
    roles
        .iter()
        .map(|role| format!("ROLE_{}", role.name.to_ascii_uppercase()))
        .collect::<Vec<String>>()
}
