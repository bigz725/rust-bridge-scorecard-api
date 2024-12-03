use async_graphql::SimpleObject;
use bcrypt::BcryptError;
use chrono::{Utc, DateTime};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::{fmt::{Display, Formatter, Result as FmtResult}, str::FromStr
};
use sqlx::PgPool;
//use tokio_stream::StreamExt;

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    // !!! ATTENTION !!!
    // When getting users from Mongodb, you must handle the roles.
    // The find_user method does this through an aggregation pipeline.
    // If you add another search method, you must handle the roles somehow.
    // Otherwise, you will get a deserialization error, and it will complain
    // about a missing "_id" field, but won't tell you it comes from the vec of roles.
    #[serde(skip_serializing)]
    pub roles: Vec<Role>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    pub roles: Vec<Role>,
    #[serde(
        //serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "createdAt"
    )]
    pub created_at: DateTime<Utc>,
    #[serde(
        //serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "updatedAt"
    )]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct Role {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum UserError {
    NoDbConnectionError,
    QueryError(#[from] mongodb::error::Error),
    InvalidUserRecord(#[from] bson::de::Error),
    BadDecryption(#[from] BcryptError),
    InvalidCredentials,
    UserNotFound,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.id, self.name)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "({}, {}, {}, {},)",
            self.id, self.username, self.email, self.created_at
        )
    }
}

impl core::fmt::Display for UserError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn all_users(db: &PgPool) -> Result<Vec<User>, UserError> {
    todo!()
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn find_user(
    db: &PgPool,
    user_id: Option<&str>,
    username: Option<&str>,
    email: Option<&str>,
    salt: Option<&str>,
) -> Result<Vec<User>, UserError> {
    todo!()
}

pub async fn save_user(db: &PgPool, user: NewUser) -> Result<(), UserError> {
    todo!()
}

pub async fn update_user(db: &PgPool, user: &User) -> Result<(), UserError> {
    todo!()
}
