use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::{Value, json};

use crate::models::user::UserError;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error{    
    LoginFail(#[from] UserError),
    
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter,) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::LoginFail(_)  => {
                (StatusCode::UNAUTHORIZED, unable_to_login_json()).into_response()
            }
        }
    }
}

fn unable_to_login_json () -> Json<Value> {
    Json(json!({"message": "Unable to login"}))
}