use std::fmt::Debug;

use sqlx::PgPool;

use crate::auth::jwt::Keys;

type DieselPgPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
#[derive(Clone)]
pub struct AppState {
    pub db_conn: PgPool,
    pub diesel_conn: DieselPgPool,
    pub keys: Keys,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("pgpool", &self.db_conn)
            .finish()
    }
}
