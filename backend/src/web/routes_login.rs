use crate::{Error, Result, AppState, auth::jwt::{Claims, create_token}};
use axum::{Json, Router, routing::post, extract::State};
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
    let user = find_user_by_username(db, &payload.username).await.expect("no user");
    match verify(&payload.password, &user.password) {
        Ok(valid) => {
            if valid {
                let claims = Claims {
                    id: user.id.to_string(), //user.id.to_string(),
                    salt: user.salt.clone(),
                    exp: 24 * 60 * 60,
                };
                let token = create_token(&claims);
                println!("User {} successfully logged in", user.username);
                let doc = json!({
                    "id": user.id.to_string(),
                    "username": user.username,
                    "email": user.email,
                    "accessToken": token,
                    "roles": user.roles.iter().map(|role| role.to_string()).collect::<Vec<String>>(),
                });

                Ok(Json(doc))
            }
            else {
                println!("Password is invalid");
                Err(Error::InvalidCredentials)
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(Error::LoginFail)
        }

    }

}
