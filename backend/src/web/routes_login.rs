use crate::{
    auth::jwt::{create_token, Claims},
    models::user::Role,
    AppState, Error, Result,
};
use axum::{extract::State, routing::post, Json, Router};
//use extract::State;
use crate::models::user::find_user_by_username;
use bcrypt::verify;
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::{json, Value};
use chrono::{Duration, Utc, Days};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/api/auth/signin", post(login))
}

async fn login(State(state): State<AppState>, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    let db = &(state.mongodb_client);

    let user = find_user_by_username(db, &payload.username).await;
    let user = match user {
        Ok(user) => user,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(Error::LoginFail);
        }
    };
    match verify(&payload.password, &user.password) {
        Ok(valid) => {
            if valid {
                let now = Utc::now().timestamp();
                let claims = Claims {
                    id: user.id.to_string(), //user.id.to_string(),
                    salt: user.salt.clone(),
                    exp: add_time(now, 1),
                };
                let token = create_token(&claims);
                println!("User {} successfully logged in", user.username);
                let doc = json!({
                    "id": user.id.to_string(),
                    "username": user.username,
                    "email": user.email,
                    "roles": process_roles(user.roles),
                    "accessToken": token,
                });

                Ok(Json(doc))
            } else {
                println!("Password is invalid");
                Err(Error::LoginFail)
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(Error::LoginFail)
        }
    }
}

fn process_roles(roles: Vec<Role>) -> Vec<String> {
    roles
        .iter()
        .map(|role| format!("ROLE_{}", role.name.to_ascii_uppercase()))
        .collect::<Vec<String>>()
}

fn add_time(now: i64, days: i64) -> usize {
    let sixty_four = now + (days * 86400);
    sixty_four as usize
}
