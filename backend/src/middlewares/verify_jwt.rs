use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Validation};

use crate::auth::jwt::{Claims, KEYS};
struct JWTDecryptError;

pub struct BearerToken(String);

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

#[tracing::instrument(skip(bearer_token, request, next))]
pub async fn get_claims_from_auth_token(
    bearer_token: BearerToken,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    let bearer_token = bearer_token.0.replace("Bearer ", "");
    match get_claims(&bearer_token) {
        Ok(claims) => {
            request.extensions_mut().insert::<Claims>(claims.clone());
            next.run(request).await
        }
        Err(_) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".into())
            .unwrap(),
    }
}
#[tracing::instrument(skip(token))]
fn get_claims(token: &str) -> Result<Claims, JWTDecryptError> {
    let decoded = decode::<Claims>(token, &KEYS.decoding, &Validation::default());

    match decoded {
        Ok(claims) => Ok(claims.claims),
        Err(err) => {
            tracing::error!("Error decoding JWT: {:?}", err);
            Err(JWTDecryptError)
        }
    }
}
