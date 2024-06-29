use mongodb::Client;

use crate::models::user::{update_user, User, UserError};

use super::salt::salt;


#[derive(thiserror::Error, Debug)]
pub enum LogoutError {
    #[error("User not found")]
    UserNotFound,
    #[error("Something's gone wrong")]
    UnexpectedError(#[from] UserError),
}

#[tracing::instrument(target="logout", skip(db))]
pub async fn logout(db: &Client, user: &mut User) -> Result<(), LogoutError> {
    user.salt = salt().await;
    update_user(db, user).await?;
    Ok(())
}