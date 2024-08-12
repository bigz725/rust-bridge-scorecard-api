use axum::{
    body::Body, extract::{Path, Request}, http::StatusCode, middleware::Next, response::Response, Extension
};

use crate::models::user::User;
/*
pub async fn lookup_user_from_token(
    Extension(claims): Extension<Claims>,
    State(AppState{ mongodb_client, keys: _}): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
 */
#[tracing::instrument(skip(user, next, request))]
pub async fn session_owner_guard(
    Extension(user): Extension<User>,
    Path(user_id): Path<String>,
    request: Request,
    next: Next,
) -> Response<Body> {
    if user.id.to_string() == user_id {
        return next.run(request).await;
    }
    tracing::warn!("User {} tried to access user {}'s session", user.id, user_id);
    Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".into())
            .unwrap()


}