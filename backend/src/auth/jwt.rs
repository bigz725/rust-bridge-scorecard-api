use jsonwebtoken::{EncodingKey, DecodingKey, encode, Header};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub salt: String,
    pub exp: usize,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or("fortestingonly".to_string());
    Keys::new(secret.as_bytes())
});


pub fn create_token(claims: &Claims) -> String {
    encode(&Header::default(), &claims, &KEYS.encoding).unwrap_or_else(|_| 
        { 
            error!("Could not encode the token."); 
            panic!("Could not encode the token."); 
        }
    )
}






