
use async_graphql::{Context, Object};
use serde_json::{json, Value};
use sqlx::PgPool;
use crate::{auth::{jwt::Keys, login::{login, LoginError, LoginPayload, LoginResponse}, logout::{logout, LogoutError}}, models::user::{all_users, find_user, User, UserError}};


pub struct Query;

pub struct Mutation;

#[Object]
impl Query {
    #[tracing::instrument(target="graphql",skip(self, context))]
    pub async fn users(&self, context: &Context<'_>) -> Result<Vec<User>, UserError> {
        let db = context.data::<PgPool>().map_err(|_| UserError::NoDbConnectionError)?;
        all_users(db).await        
    }
    #[tracing::instrument(target="graphql",skip(self, context))]
    pub async fn user(&self, context: &Context<'_>, username: String) -> Result<Vec<User>, UserError> {
        let db = context.data::<PgPool>().map_err(|_| UserError::NoDbConnectionError)?;
        //find_user(db: &Client, user_id: Option<&str>, username: Option<&str>, email: Option<&str>, salt: Option<&str>)
        find_user(db, None, Some(username).as_deref(), None, None).await
    }
}

#[Object]
impl Mutation {
    pub async fn login(&self, context: &Context<'_>, payload: LoginPayload) -> Result<LoginResponse, LoginError> {
        let db = context.data::<PgPool>().expect("No db connection");
        let keys = context.data::<Keys>().expect("No keys");
        let response = login(db, keys, payload).await?;

        Ok(response)

    }

    pub async fn logout(&self, ctx: &Context<'_>) -> Result<Value, LogoutError> {
        let db = ctx.data::<PgPool>().expect("No db connection");
        let user = ctx.data::<Option<User>>()
            .map_err(|_| {
                tracing::error!("Error retrieving user from context");
                LogoutError::UserNotFound
            })?;
        match user {
            Some(ref user) => {
                tracing::info!("Logging out user: {}", user.username);
                let mut target = user.clone();
                logout(db, &mut target).await?;
                Ok(json!({"message": format!("User: {} logged out", user.username)}))
            }
            None => {
                tracing::info!("Logging out user: No user found in context");
                Err(LogoutError::UserNotFound)
            }
        
        }
        
        
    }
}