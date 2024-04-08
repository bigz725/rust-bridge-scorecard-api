use crate::{
    auth::jwt::{create_token, Claims},
    models::user::{find_user_by_username, Role, User, UserError, UserError::InvalidCredentials},
    state::AppState, error::Error,
};
use axum::{extract::State, routing::post, Json, Router};
use bcrypt::verify;
use chrono::Utc;
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::{info, instrument, warn};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/signin", post(login))
}

#[instrument(name = "Login", skip(state, payload))]
async fn login(
    State(state): State<AppState>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>, Error> {
    let db = &(state.mongodb_client);
    let keys = &state.keys;

    let user = find_user_by_username(db, &payload.username).await?;
    let verify_result =
        verify(&payload.password, &user.password).map_err(UserError::BadDecryption)?;

    if verify_result {
        let claims = create_claims(&user);
        let token = create_token(&claims, &keys.encoding);
        info!("User {} successfully logged in", user.username);
        let response = response(user, token);

        Ok(response)
    } else {
        warn!("Password incorrect for user {}", payload.username);
        Err(Error::LoginFail(InvalidCredentials))
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
