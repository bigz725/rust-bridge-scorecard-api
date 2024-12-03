use async_graphql::{Enum, SimpleObject};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Invalid scoring type string: {0}")]
    InvalidScoringTypeString(String),
    #[error("No database connection")]
    NoDbConnectionError,
    #[error("Query error: {0}")]
    QueryError(#[from] mongodb::error::Error),
    #[error("Invalid session record: {0}")]
    InvalidSessionRecord(#[from] bson::de::Error),
    #[error("Could not convert {0} to ObjectId")]
    InvalidObjectId(#[from] bson::oid::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub date: DateTime<Utc>,
    pub owner: Uuid,
    pub scoring_type: ScoringType,
    pub should_use_victory_points: bool,
}
#[derive(Debug, Serialize, Deserialize, Enum, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ScoringType {
    Imp,
    Mp,
}

impl std::fmt::Display for ScoringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoringType::Imp => write!(f, "IMP"),
            ScoringType::Mp => write!(f, "MP"),
        }
    }
}

impl std::str::FromStr for ScoringType {
    type Err = SessionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IMP" => Ok(ScoringType::Imp),
            "MP" => Ok(ScoringType::Mp),
            _ => Err(SessionError::InvalidScoringTypeString(s.to_string())),
        }
    }
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn get_sessions(db: &PgPool, scoring_type: Option<ScoringType>) -> Result<Vec<Session>, SessionError> {
    todo!()
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn get_sessions_for_user_id(
    db: &PgPool,
    user_id: &Uuid,
    scoring_type: Option<ScoringType>,
) -> Result<Vec<Session>, SessionError> {
    todo!()
}
#[tracing::instrument(target = "database", skip(db))]
pub async fn create_session(db: &PgPool, session: Session) -> Result<String, SessionError> {
    todo!()
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn update_session(
    db: &PgPool,
    session_id: &Uuid,
    session_update: Session,
) -> Result<(), SessionError> {
    todo!()
}



