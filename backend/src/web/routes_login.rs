use crate::{Error, Result, AppState};
use axum::{Json, Router, routing::post, extract::State};
//use extract::State;
use serde::Deserialize;
use serde_json::{Value, json};
use mongodb::{bson::doc, Collection};
use crate::models::user::{User, find_user_by_username};
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
                    println!("Password is valid");
                    Ok(Json(json!({
                        "message": "Found user",
                        "user": user
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
        println!("User not found");
        Err(Error::InvalidCredentials)
    }

}
