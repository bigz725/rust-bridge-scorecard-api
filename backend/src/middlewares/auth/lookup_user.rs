use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::{auth::jwt::Claims, models::user::find_user_by_uuid, state::AppState, auth::login::LoginError};

#[tracing::instrument(skip(claims, diesel, request, next))]
pub async fn lookup_user_from_token(
    Extension(claims): Extension<Claims>,
    State(AppState{ db_conn: _, diesel_conn: diesel, keys: _}): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    //find_user(db: &Client, user_id: Option<&str>, username: Option<&str>, email: Option<&str>, salt: Option<&str>)
    let result = find_user_by_uuid(&diesel, &claims.id, Some(&claims.salt))
        .await
        .map_err(LoginError::from);
    match result {
        Ok(user) => {
            tracing::info!("User {} successfully looked up", user.username.clone());
            request.extensions_mut().insert(user.to_owned());
            next.run(request).await
        }
        Err(err) => {
            tracing::error!(
                "Error looking up user with id: {} and salt: {}",
                &claims.id,
                &claims.salt
            );
            return err.into_response();
        }
    }
}
//sdfsdf