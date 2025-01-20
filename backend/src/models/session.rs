use async_graphql::SimpleObject;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use super::scoring_type::ScoringTypeEnum;
type DieselPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Invalid scoring type string: {0}")]
    InvalidScoringTypeString(String),
    #[error("No database connection")]
    NoDbConnectionError,
}

/*
#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
 */

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, Queryable, Selectable, Identifiable, SimpleObject)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub location: Option<String>,
    pub date: NaiveDate,
    pub owner_id: Uuid,
    pub scoring_type: ScoringTypeEnum,
    pub should_use_victory_points: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}



#[tracing::instrument(target = "database", skip(db))]
pub async fn get_sessions(db: &DieselPool, scoring_type: Option<ScoringTypeEnum>) -> Result<Vec<Session>, SessionError> {
    todo!()
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn get_sessions_for_user_id(
    db: &DieselPool,
    user_id: &Uuid,
    scoring_type: Option<ScoringTypeEnum>,
) -> Result<Vec<Session>, SessionError> {
    todo!()
}
#[tracing::instrument(target = "database", skip(db))]
pub async fn create_session(db: &DieselPool, session: Session) -> Result<String, SessionError> {
    todo!()
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn update_session(
    db: &DieselPool,
    session_id: &Uuid,
    session_update: Session,
) -> Result<(), SessionError> {
    todo!()
}



