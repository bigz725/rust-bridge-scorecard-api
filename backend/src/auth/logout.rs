use crate::models::user::{update_user, UpdateUser, User, UserError};

use super::salt::salt;
type DieselPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(thiserror::Error, Debug)]
pub enum LogoutError {
    #[error("User not found")]
    UserNotFound,
    #[error("Something's gone wrong")]
    UnexpectedError(#[from] UserError),
}

#[tracing::instrument(target="logout", skip(db))]
pub async fn logout(db: &DieselPool, user: &User) -> Result<(), LogoutError> {
    let salt = salt().await;
    let update = UpdateUser {
        id: user.id,
        salt: Some(salt),
        ..Default::default()
    };
    update_user(db, update).await?;
    Ok(())
}