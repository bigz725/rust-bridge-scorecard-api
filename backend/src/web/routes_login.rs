use crate::{Error, Result, AppState, auth::jwt::{Claims, KEYS, create_token}};
use axum::{Json, Router, routing::post, extract::State};
use jsonwebtoken::{encode, Header};
//use extract::State;
use serde::Deserialize;
use serde_json::{Value, json};
use mongodb::bson::doc;
use crate::models::user::find_user_by_username;
use bcrypt::verify;
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/signin", post(login))

}

async fn login(State(state): State<AppState>, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    let db = &(state.mongodb_client);
    let user = find_user_by_username(db, &payload.username).await;
    if let Some(user) = user {
        println!("Found user: {:?}", user);
        match verify(&payload.password, &user.password) {
            Ok(valid) => {
                if valid {
                    
                    let claims = Claims {
                        id: user.id.to_string(),
                        salt: user.salt.clone(),
                        exp: 24 * 60 * 60,
                    };
                    let token = create_token(&claims);

                    println!("User {} successfully logged in", user.username);
                    Ok(Json(json!({
                        "id": user.id.to_string(),
                        "username": user.username,
                        "email": user.email,
                        "accessToken": token
                    })))
                } else {
                    println!("Password is invalid");
                    Err(Error::InvalidCredentials)
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                Err(Error::LoginFail)
            }
        }
    } else {
        println!("Login attempt for non-existant user {}", payload.username);
        Err(Error::InvalidCredentials)
    }

}
