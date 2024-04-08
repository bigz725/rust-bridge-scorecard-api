use std::fmt::Debug;

use mongodb::Client;

use crate::auth::jwt::Keys;
#[derive(Clone)]
pub struct AppState {
    pub mongodb_client: Client,
    pub keys: Keys,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("mongodb_client", &self.mongodb_client)
            .finish()
    }
}
