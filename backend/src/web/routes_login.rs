use crate::{Error, Result, AppState};
use axum::{Json, Router, routing::post};

use serde::Deserialize;
use serde_json::{Value, json};


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/signin", post(login))

}

async fn login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    let body = Json(json!({
        "result": {
            "success": true,
            "username": payload.username,
            "password": payload.password,
        }
    }));
    Ok(body)
}