use diesel::{deserialize::FromSqlRow, expression::AsExpression};
use async_graphql::Enum;
use super::session::SessionError;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql, Result as SerializeResult};
use diesel::deserialize::{FromSql, Result as DeserializeResult};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::schema::sql_types::ScoringType;

// #[derive(SqlType)]
// #[diesel(postgres_type(name = "scoring_type"))]
// pub struct ScoringType;

#[derive(Debug, Deserialize, Serialize, PartialEq, FromSqlRow, AsExpression, Eq, Clone, Enum, Copy)]
#[diesel(sql_type=ScoringType)]
pub enum ScoringTypeEnum {
    Imp,
    Mp,
}

impl std::fmt::Display for ScoringTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoringTypeEnum::Imp => write!(f, "IMP"),
            ScoringTypeEnum::Mp => write!(f, "MP"),
        }
    }
}

impl ToSql<ScoringType, Pg> for ScoringTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
        match self {
            ScoringTypeEnum::Imp => out.write_all(b"IMP")?,
            ScoringTypeEnum::Mp => out.write_all(b"MP")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ScoringType, Pg> for ScoringTypeEnum {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
        match bytes.as_bytes() {
            b"IMP" => Ok(ScoringTypeEnum::Imp),
            b"MP" => Ok(ScoringTypeEnum::Mp),
            _ => Err(SessionError::InvalidScoringTypeString(String::from_utf8_lossy(bytes.as_bytes()).to_string()).into()),
        }
    }
}