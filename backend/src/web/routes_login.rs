use crate::{
    auth::jwt::{create_token, Claims},
    models::user::{find_user_by_username,Role, User, UserError, UserError::InvalidCredentials},
    AppState, Error, 
};
use axum::{extract::State, routing::post, Json, Router};
use bcrypt::verify;
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::{json, Value};
use chrono::Utc;

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/signin", post(login))
}

async fn login(State(state): State<AppState>, payload: Json<LoginPayload>) -> Result<Json<Value>, Error> {
    let db = &(state.mongodb_client);

    let user = find_user_by_username(db, &payload.username).await?;
    let verify_result = verify(&payload.password, &user.password).map_err( UserError::BadDecryption)?;

    if verify_result {
        let claims = get_claims(&user);
        let token = create_token(&claims);
        println!("User {} successfully logged in", user.username);
        let response = response(user, token);

        Ok(response)
    } else {
        println!("Password incorrect for user {}", payload.username);
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

fn get_claims(user: &User) -> Claims {
    let now = Utc::now().timestamp();
    Claims {
        id: user.id.to_string(),
        salt: user.salt.clone(),
        exp: add_time(now, 1),
    }
}

fn add_time(now: i64, days: i64) -> usize {
    let sixty_four = now + (days * 86400);
    sixty_four as usize
}
