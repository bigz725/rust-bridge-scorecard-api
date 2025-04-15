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
    IMP,
    MP,
}

impl std::fmt::Display for ScoringTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoringTypeEnum::IMP => write!(f, "IMP"),
            ScoringTypeEnum::MP => write!(f, "MP"),
        }
    }
}

impl ToSql<ScoringType, Pg> for ScoringTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
        match self {
            ScoringTypeEnum::IMP => out.write_all(b"IMP")?,
            ScoringTypeEnum::MP => out.write_all(b"MP")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ScoringType, Pg> for ScoringTypeEnum {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
        match bytes.as_bytes() {
            b"IMP" => Ok(ScoringTypeEnum::IMP),
            b"MP" => Ok(ScoringTypeEnum::MP),
            _ => Err(SessionError::InvalidScoringTypeString(String::from_utf8_lossy(bytes.as_bytes()).to_string()).into()),
        }
    }
}