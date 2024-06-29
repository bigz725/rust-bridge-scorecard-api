use async_graphql::SimpleObject;
use bcrypt::BcryptError;
use bson::{oid::ObjectId, serde_helpers::serialize_bson_datetime_as_rfc3339_string, Bson, Document};
use mongodb::{
    bson::{doc, DateTime},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::{fmt::{Display, Formatter, Result as FmtResult}, str::FromStr
};
//use tokio_stream::StreamExt;
use futures::stream::TryStreamExt;

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct User {
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
    #[serde(skip_serializing)]
    pub roles: Vec<Role>,

    #[serde(
        //serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "createdAt"
    )]
    pub created_at: DateTime,
    #[serde(
        //serialize_with = "serialize_bson_datetime_as_rfc3339_string",
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
        //serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "createdAt"
    )]
    pub created_at: DateTime,
    #[serde(
        //serialize_with = "serialize_bson_datetime_as_rfc3339_string",
        rename = "updatedAt"
    )]
    pub updated_at: DateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
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

impl From<User> for Bson {
    fn from(user: User) -> Bson {
        bson::to_bson(&user).unwrap()
    }
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn all_users(db: &Client) -> Result<Vec<User>, UserError> {
    let users: Collection<User> = db.database("bridge_scorecard_api").collection("users");
    let pipeline = vec![
        stage_lookup_roles(),
    ];
    do_vec_aggregation(users, pipeline).await
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn find_user(
    db: &Client,
    user_id: Option<&str>,
    username: Option<&str>,
    email: Option<&str>,
    salt: Option<&str>,
) -> Result<Vec<User>, UserError> {
    let users: Collection<User> = db.database("bridge_scorecard_api").collection("users");
    let pipeline = vec![
        stage_lookup_user(user_id, username, email, salt),
        stage_lookup_roles(),
    ];
    do_vec_aggregation(users, pipeline).await
}

pub async fn save_user(db: &Client, user: NewUser) -> Result<(), UserError> {
    let users: Collection<NewUser> = db.database("bridge_scorecard_api").collection("users");
    users.insert_one(user, None).await?;
    Ok(())
}

pub async fn update_user(db: &Client, user: &User) -> Result<(), UserError> {
    let users: Collection<User> = db.database("bridge_scorecard_api").collection("users");
    users.update_one(
        doc! {"_id": user.id},
        doc! {"$set": user},
        None,
    )
    .await?;
    Ok(())
}

#[tracing::instrument(target = "database", skip(users), level = "trace")]
async fn do_vec_aggregation(
    users: Collection<User>,
    pipeline: Vec<Document>,
) -> Result<Vec<User>, UserError> {
    let mut cursor = users.aggregate(pipeline, None).await?;
    let mut results: Vec<User> = Vec::new();

    while let Some(document) = cursor.try_next().await? {
        let bson = bson::from_document(document)
            .map_err(|e| {
                tracing::error!("Error in from_document: {:?}", e);
                e
            })?;
        tracing::info!("{:?}", bson);
        let user: User = bson::from_bson(bson)
            .map_err(|e| {
                tracing::error!("Error in from_bson: {:?}", e);
                e
            })?;
        results.push(user);
    }
    Ok(results)
    
}

fn stage_lookup_user(
    user_id: Option<&str>,
    username: Option<&str>,
    email: Option<&str>,
    salt: Option<&str>,
) -> Document {
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
