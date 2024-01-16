use bcrypt::BcryptError;
use mongodb::{bson::{DateTime, doc}, Client, Collection};
use serde::{Deserialize, Serialize};
use bson::{serde_helpers::serialize_bson_datetime_as_rfc3339_string, oid::ObjectId, Document};
use tokio_stream::StreamExt;
use std::fmt::{Display, Formatter, Result as FmtResult};
use tracing::warn;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    // #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    pub roles: Vec<Role>,

    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", rename = "updatedAt")]
    pub updated_at: DateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub  salt: String,
    pub email: String,
    pub roles: Vec<Role>,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", rename = "createdAt")]
    pub updated_at: DateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Role {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", rename = "updatedAt")]
    pub updated_at: DateTime,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum UserError {
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
        write!(f, "({}, {}, {}, {},)", self.id, self.username, self.email, self.created_at)
    }
}

impl core::fmt::Display for UserError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter,) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

pub async fn find_user_by_username(db: &Client, username: &str) -> Result<User, UserError> {
    let users: Collection<User> = db.database("bridge_scorecard_api").collection("users");
    let mut results  = users.aggregate([
            stage_lookup_user(username),
            stage_lookup_roles(),], None
    ).await?;

    let result = results.try_next().await?;
    if let Some(result) = result {
        let doc = bson::from_document(result)?;
        let user: User = bson::from_bson(doc)?;
        Ok(user)
    } else {
        warn!("User {} not found", username);
        Err(UserError::UserNotFound)
    }
}


fn stage_lookup_user(username: &str) -> Document {
    doc!{
        "$match": doc! {
            "username": &(username),
        }
    }
}

fn stage_lookup_roles() -> Document {
    doc! {
        "$lookup": doc!{
            "from": "roles",
            "localField": "roles",
            "foreignField": "_id",
            "as": "roles"
        }
    }
}
