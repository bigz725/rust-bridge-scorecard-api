use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::{auth::jwt::Claims, models::user::find_user, state::AppState, web::routes_login::LoginError};

#[tracing::instrument(skip(claims, mongodb_client, request, next))]
pub async fn lookup_user_from_token(
    Extension(claims): Extension<Claims>,
    State(AppState{ mongodb_client, keys: _}): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    //find_user(db: &Client, user_id: Option<&str>, username: Option<&str>, email: Option<&str>, salt: Option<&str>)
    let user = find_user(&mongodb_client,Some(&claims.id), None, None, Some(&claims.salt)).await;
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
            LoginError::from(err).into_response()
        }
    }
}
