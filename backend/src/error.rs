use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::{Value, json};


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    LoginFail,
    InvalidCredentials,
}

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter,) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::LoginFail | Error::InvalidCredentials => {
                (StatusCode::NOT_FOUND, unable_to_login_json()).into_response()
            }
        }
        // println!("->> {:12} - {self:?}", "INTO_RES");
        // (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

fn unable_to_login_json () -> Json<Value> {
    Json(json!({"message": "Unable to login"}))
}