use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Extension,
};

use crate::{auth::jwt::Claims, models::user::find_by_user_id_and_salt, AppState};

#[tracing::instrument(skip(ext, state, next))]
pub async fn lookup_user_from_token(
    ext: Extension<Claims>,
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    let db = state.mongodb_client;
    let Extension(claims) = ext;
    let user = find_by_user_id_and_salt(&db, &claims.id, &claims.salt).await;
    match user {
        Ok(user) => {
            tracing::info!("User {} successfully looked up", user.username);
            request.extensions_mut().insert(user);
            next.run(request).await
        }
        Err(err) => {
            tracing::error!(
                "Error looking up user with id: {} and salt: {}",
                &claims.id,
                &claims.salt
            );
            tracing::error!("Error was: {:?}", err);
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("Unauthorized".into())
                .unwrap()
        }
    }
}
