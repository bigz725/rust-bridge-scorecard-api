use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use thiserror::Error;
use crate::{auth::jwt::Claims, state::AppState};

#[derive(Debug, Error)]
#[error("Error decrypting JWT")]

pub struct JWTDecryptError;

impl IntoResponse for JWTDecryptError {
    fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".into())
            .unwrap()
    }
}

pub struct BearerToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(authorization) = parts.headers.get("Authorization") {
            let token_string = authorization
                .to_str()
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized"))?
                .to_string();
            Ok(BearerToken(token_string))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
        }
    }
}

#[tracing::instrument(skip(bearer_token, keys, request, next))]
pub async fn get_claims_from_auth_token(
    bearer_token: BearerToken,
    State(AppState{mongodb_client: _, keys}): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    let bearer_token = bearer_token.0.replace("Bearer ", "");
    let decoding_key = keys.decoding;
    match get_claims(&bearer_token, &decoding_key) {
        Ok(claims) => {
            request.extensions_mut().insert::<Claims>(claims.clone());
            next.run(request).await
        }
        Err(e) => {
            tracing::error!("Error decoding JWT: {:?}", e);
            e.into_response()
        }
    }
}
#[tracing::instrument(skip(token, decoding_key))]
pub fn get_claims(token: &str, decoding_key: &DecodingKey) -> Result<Claims, JWTDecryptError> {
    let decoded = decode::<Claims>(token, decoding_key, &Validation::default());

    match decoded {
        Ok(claims) => Ok(claims.claims),
        Err(err) => {
            tracing::error!("Error decoding JWT: {:?}", err);
            Err(JWTDecryptError)
        }
    }
}
