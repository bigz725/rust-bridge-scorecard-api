use crate::{Error, Result};
use axum::{Json, Router, routing::post};

use serde::Deserialize;
use serde_json::{Value, json};


#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
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
    // let body = Json(json!({
    //     "result": "success"
    // }));
    Ok(body)
}