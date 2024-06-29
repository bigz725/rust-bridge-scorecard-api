use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::{auth::jwt::Claims, models::user::find_user, state::AppState, auth::login::LoginError, models::user::UserError};

#[tracing::instrument(skip(claims, mongodb_client, request, next))]
pub async fn lookup_user_from_token(
    Extension(claims): Extension<Claims>,
    State(AppState{ mongodb_client, keys: _}): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    //find_user(db: &Client, user_id: Option<&str>, username: Option<&str>, email: Option<&str>, salt: Option<&str>)
    let result = find_user(&mongodb_client,Some(&claims.id), None, None, Some(&claims.salt))
        .await
        .map_err(LoginError::from);
    let users = match result {
        Ok(users) => users,
        Err(err) => return err.into_response(),
    };
    let user = users.first();
    match user {
        Some(user) => {
            tracing::info!("User {} successfully looked up", user.username.clone());
            request.extensions_mut().insert(user.to_owned());
            next.run(request).await
        }
        None => {
            tracing::error!(
                "Error looking up user with id: {} and salt: {}",
                &claims.id,
                &claims.salt
            );
            LoginError::from(UserError::UserNotFound).into_response()
        }
    }
}
