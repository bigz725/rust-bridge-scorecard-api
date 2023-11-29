use mongodb::{bson::{DateTime, doc}, Client, Collection,};
use serde::{Deserialize, Serialize};
use bson::serde_helpers::{serialize_hex_string_as_object_id, serialize_bson_datetime_as_rfc3339_string};
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    // #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    // pub _id: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: String,

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
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", rename = "createdAt")]
    pub updated_at: DateTime,
}


pub async fn find_user_by_username(db: &Client, username: &str) -> Option<User> {
    let users: Collection<User> = db.database("bridge_scorecard_api").collection("users");
    let user = users.find_one(
        {
            doc!{
                "username": &(username),
            }
        }, None
    ).await;
    match user {
        Ok(user) => {
            user
        }
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    }
}
