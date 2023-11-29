use axum::{response::{IntoResponse, Response}, http::StatusCode};


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
            Error::LoginFail => {
                println!("->> {:12} - {self:?}", "INTO_RES");
                (StatusCode::INTERNAL_SERVER_ERROR, "LOGIN_FAIL").into_response()
            }
            Error::InvalidCredentials => {
                println!("->> {:12} - {self:?}", "INTO_RES");
                (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS").into_response()
            }
        
        }
        // println!("->> {:12} - {self:?}", "INTO_RES");
        // (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}