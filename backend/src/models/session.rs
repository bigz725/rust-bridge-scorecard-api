use async_graphql::{Enum, SimpleObject};
use bson::{
    oid::ObjectId, DateTime, Document
};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionMongoDTO {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub location: String,
    pub date: DateTime,
    pub owner: ObjectId,
    pub scoring_type: ScoringType,
    pub should_use_victory_points: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionDTO {
    pub name: String,
    pub location: Option<String>,
    pub date: Option<String>,
    pub owner: String,
    pub scoring_type: ScoringType,
    pub should_use_victory_points: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionUpdateDTO {
    pub name: Option<String>,
    pub location: Option<String>,
    pub date: Option<String>,
    pub scoring_type: Option<ScoringType>,
    pub should_use_victory_points: Option<bool>,
}

impl From<SessionUpdateDTO> for Document {
    fn from(session_update: SessionUpdateDTO) -> Self {
        let mut updates = doc! {};
        if let Some(name) = session_update.name {
            updates.insert("name", name);
        }
        if let Some(location) = session_update.location {
            updates.insert("location", location);
        }
        if let Some(date) = session_update.date {
            let chrono_dt: chrono::DateTime<Utc> = date.parse().unwrap();
            let bson_dt: bson::DateTime = chrono_dt.into();
            updates.insert("date", bson_dt);
        }
        if let Some(scoring_type) = session_update.scoring_type {
            updates.insert("scoringType", scoring_type.to_string());
        }
        if let Some(should_use_victory_points) = session_update.should_use_victory_points {
            updates.insert("shouldUseVictoryPoints", should_use_victory_points);
        }
        doc! {
            "$set": updates
        }
    }

}
#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct SessionJsonDTO {
    pub id: String,
    pub name: String,
    pub location: String,
    pub date: String,
    pub owner: String,
    pub scoring_type: ScoringType,
    pub should_use_victory_points: bool,
}

impl From<SessionMongoDTO> for SessionJsonDTO {
    fn from(session: SessionMongoDTO) -> Self {
        SessionJsonDTO {
            id: session.id.to_string(),
            name: session.name,
            location: session.location,
            date: session.date.to_string(),
            owner: session.owner.to_string(),
            scoring_type: session.scoring_type,
            should_use_victory_points: session.should_use_victory_points,
        }
    }
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
pub async fn get_sessions(db: &Client, scoring_type: Option<ScoringType>) -> Result<Vec<SessionJsonDTO>, SessionError> {
    let collection: Collection<SessionMongoDTO> =
        db.database("bridge_scorecard_api").collection("sessions");
    let pipeline = vec![stage_lookup_session(None, scoring_type)];
    let mut sessions: Vec<SessionJsonDTO> = Vec::new();
    let mut cursor = collection.aggregate(pipeline).await?;
    while let Some(document) = cursor.try_next().await? {
        let session: SessionJsonDTO = bson::from_document::<SessionMongoDTO>(document)
            .map_err(|e| {
                tracing::error!("Error in from_document: {:?}", e);
                e
            })?
            .into();
        sessions.push(session);
    }

    Ok(sessions)
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn get_sessions_for_user_id(
    db: &Client,
    user_id: &ObjectId,
    scoring_type: Option<ScoringType>,
) -> Result<Vec<SessionJsonDTO>, SessionError> {
    let collection: Collection<SessionMongoDTO> =
        db.database("bridge_scorecard_api").collection("sessions");
    let pipeline = vec![stage_lookup_session(Some(user_id), scoring_type)];
    let mut sessions: Vec<SessionJsonDTO> = Vec::new();
    let mut cursor = collection.aggregate(pipeline).await?;
    while let Some(document) = cursor.try_next().await? {
        let session: SessionJsonDTO = bson::from_document::<SessionMongoDTO>(document)
            .map_err(|e| {
                tracing::error!("Error in from_document: {:?}", e);
                e
            })?
            .into();
        sessions.push(session);
    }

    Ok(sessions)
}
#[tracing::instrument(target = "database", skip(db))]
pub async fn create_session(db: &Client, session: NewSessionDTO) -> Result<String, SessionError> {
    let collection: Collection<NewSessionDTO> =
        db.database("bridge_scorecard_api").collection("sessions");
    let insert_result = collection.insert_one(session).await?;
    let inserted_id = insert_result
        .inserted_id
        .as_object_id()
        .unwrap()
        .to_string();
    tracing::info!("Created session id: {:?}", inserted_id);
    Ok(inserted_id)
}

#[tracing::instrument(target = "database", skip(db))]
pub async fn update_session(
    db: &Client,
    session_id: &str,
    session_update: SessionUpdateDTO,
) -> Result<(), SessionError> {
    let collection: Collection<SessionMongoDTO> =
        db.database("bridge_scorecard_api").collection("sessions");
    let session_id = ObjectId::parse_str(session_id)?;
    let update: Document = session_update.into();
    collection
        .update_one(doc! { "_id": session_id }, update)
        .await?;
    tracing::info!("Updated session id: {:?}", session_id);
    Ok(())
}

fn stage_lookup_session(user_id: Option<&ObjectId>, scoring_type: Option<ScoringType>) -> Document {
    let mut filter = doc! {};
    if let Some(user_id) = user_id {
        filter.insert("owner", user_id);
    }
    if let Some(scoring_type) = scoring_type {
        filter.insert("scoring_type", scoring_type.to_string());
    }
    doc! {
        "$match": filter
    }
}


