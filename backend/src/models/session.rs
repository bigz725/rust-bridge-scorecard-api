use async_graphql::SimpleObject;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use crate::web::routes_user_session::SessionUpdatePayload;

use super::scoring_type::ScoringTypeEnum;
type DieselPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Invalid scoring type string: {0}")]
    InvalidScoringTypeString(String),
    #[error("No database connection")]
    NoDbConnectionError,
    #[error("Error querying the database")]
    DieselError(#[from] diesel::result::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, Queryable, Selectable, Identifiable, SimpleObject, AsChangeset)]
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

#[derive(Debug, Deserialize, Serialize, Clone, AsChangeset, Default)]
#[diesel(table_name = crate::schema::sessions)]
pub struct UpdateSession{
    pub id: Uuid,
    pub name: Option<String>,
    pub location: Option<String>,
    pub scoring_type: Option<ScoringTypeEnum>,
    pub should_use_victory_points: Option<bool>,
}

impl From<SessionUpdatePayload> for UpdateSession {
    fn from(payload: SessionUpdatePayload) -> Self {
        UpdateSession {
            id: payload.id,
            name: payload.name,
            location: payload.location,
            scoring_type: payload.scoring_type,
            should_use_victory_points: payload.should_use_victory_points,
        }
    }
}


#[tracing::instrument(target = "database", skip(db))]
pub async fn get_sessions(db: &DieselPool, scoring_type_param: Option<ScoringTypeEnum>) -> Result<Vec<Session>, SessionError> {
    use crate::schema::sessions::dsl::*;
    let mut conn = db.clone().get().unwrap();
    let mut query = sessions.into_boxed();
    if let Some(scoring_type_param) = scoring_type_param {
        query = query.filter(scoring_type.eq(scoring_type_param));
    }

    query
        .select(Session::as_select())
        .load::<Session>(&mut conn)
        .map_err(|e| {
            tracing::error!("Error: {:?}", e);
            SessionError::DieselError(e)
        })
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn get_sessions_for_user_id(
    db: &DieselPool,
    user_id: &Uuid,
    scoring_type_param: Option<ScoringTypeEnum>,
) -> Result<Vec<Session>, SessionError> {
    use crate::schema::sessions::dsl::*;
    let mut conn = db.clone().get().unwrap();
    let mut query = sessions.into_boxed();
    if let Some(scoring_type_param) = scoring_type_param {
        query = query.filter(scoring_type.eq(scoring_type_param));
    }

    query
        .limit(5)
        .filter(owner_id.eq(user_id))
        .select(Session::as_select())
        .load::<Session>(&mut conn)
        .map_err(|e| {
            tracing::error!("Error: {:?}", e);
            SessionError::DieselError(e)
        })

}
#[tracing::instrument(target = "database", skip(db))]
pub async fn create_session(db: &DieselPool, session: Session) -> Result<String, SessionError> {
    todo!()
}

/*     use crate::schema::users::dsl::*;
    let mut conn = db.clone().get().unwrap();
    
    diesel::update(users)
        .filter(id.eq(user_param.id))
        .set(user_param)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Error: {:?}", e);
            UserError::NoDbConnectionError
        })?;

    Ok(())
 */
#[tracing::instrument(target = "database", skip(db))]
pub async fn update_session(
    db: &DieselPool,
    session_id: &Uuid,
    session_update: Session,
) -> Result<(), SessionError> {
    use crate::schema::sessions::dsl::*;
    let mut conn = db.clone().get().unwrap();
    diesel::update(sessions)
        .filter(id.eq(session_id))
        .set(session_update)
        .execute(&mut conn)
        .map_err(|e| {
            tracing::error!("Error: {:?}", e);
            SessionError::NoDbConnectionError
        })?;
    Ok(())
}



