use async_graphql::SimpleObject;
use bcrypt::BcryptError;
use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use sqlx::PgPool;
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};

type DieselPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub salt: String,    
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Queryable, Selectable)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum UserError {
    NoDbConnectionError,
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

#[tracing::instrument(target = "database", skip(_db))]
pub async fn all_users(_db: &PgPool) -> Result<Vec<User>, UserError> {
    todo!()
}

#[tracing::instrument(target = "database", skip(diesel_pool))]
pub async fn find_user(
    diesel_pool: &DieselPool,
    user_id: Option<&str>,
    username: Option<&str>,
    email: Option<&str>,
    salt: Option<&str>,
) -> Result<Vec<User>, UserError> {
    use crate::schema::users::dsl::*;
    let mut conn = diesel_pool.clone().get().unwrap();
    

    
    let results = users
        .limit(5)
        .select(User::as_select())
        .load::<User>(&mut conn)
        .map_err(|e| {
            tracing::error!("Error: {:?}", e);
            UserError::NoDbConnectionError
        });
    results
}

pub async fn save_user(_db: &PgPool, _user: &User) -> Result<(), UserError> {
    todo!()
}

pub async fn update_user(_db: &PgPool, _user: &User) -> Result<(), UserError> {
    todo!()
}
