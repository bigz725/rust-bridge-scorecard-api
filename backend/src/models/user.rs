use bcrypt::BcryptError;
use bson::{oid::ObjectId, serde_helpers::serialize_bson_datetime_as_rfc3339_string, Document};
use mongodb::{
    bson::{doc, DateTime},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use tokio_stream::StreamExt;
use tracing::warn;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    // #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    #[serde(rename = "_id")]
    pub id: ObjectId,
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
    pub roles: Vec<Role>,

    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "createdAt"
    )]
    pub created_at: DateTime,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "updatedAt"
    )]
    pub updated_at: DateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    pub roles: Vec<Role>,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "createdAt"
    )]
    pub created_at: DateTime,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "createdAt"
    )]
    pub updated_at: DateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Role {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "createdAt"
    )]
    pub created_at: DateTime,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "updatedAt"
    )]
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

pub async fn find_user(db: &Client, user_id: Option<&str>, username: Option<&str>, email: Option<&str>, salt: Option<&str>) -> Result<User, UserError> {
    let users: Collection<User> = db.database("bridge_scorecard_api").collection("users");
    let pipeline = vec![
        stage_lookup_user(user_id, username, email, salt),
        stage_lookup_roles(),
    ];
    do_aggregation(users, pipeline).await

}

async fn do_aggregation(
    users: Collection<User>,
    pipeline: Vec<Document>,
) -> Result<User, UserError> {
    let mut results = users.aggregate(pipeline, None).await?;

    let result = results.try_next().await?;
    if let Some(result) = result {
        let doc = bson::from_document(result)?;
        let user: User = bson::from_bson(doc)?;
        Ok(user)
    } else {
        warn!("User not found");
        Err(UserError::UserNotFound)
    }
}

fn stage_lookup_user(user_id: Option<&str>, username: Option<&str>, email: Option<&str>, salt: Option<&str>) -> Document {
    let mut filter = doc! {};
    if let Some(user_id) = user_id {
        filter.insert("_id", ObjectId::from_str(user_id).unwrap());
    }
    if let Some(username) = username {
        filter.insert("username", username);
    }
    if let Some(email) = email {
        filter.insert("email", email);
    }
    if let Some(salt) = salt {
        filter.insert("salt", salt);
    }
    doc! {
        "$match": filter
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
