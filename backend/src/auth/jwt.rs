use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: String,
    pub salt: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub fn create_token(claims: &Claims, encoding_key: &EncodingKey) -> String {
    encode(&Header::default(), &claims, encoding_key).unwrap_or_else(|_| {
        error!("Could not encode the token.");
        panic!("Could not encode the token.");
    })
}
