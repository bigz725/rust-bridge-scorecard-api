use std::fmt::Debug;

use sqlx::PgPool;

use crate::auth::jwt::Keys;
#[derive(Clone)]
pub struct AppState {
    pub db_conn: PgPool,
    pub keys: Keys,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("pgpool", &self.db_conn)
            .finish()
    }
}
