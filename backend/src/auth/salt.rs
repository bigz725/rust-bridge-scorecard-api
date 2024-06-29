use rand::prelude::*;
use base64::{engine::general_purpose, Engine as _};

pub async fn salt() -> String {
    let mut rng = rand::rngs::StdRng::from_entropy();
    let mut bytes = [0u8; 40]; //TODO: Add the salt length to the config
    rng.fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(bytes)
}