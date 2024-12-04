use std::fmt::Display;

use chrono::{Local, NaiveDateTime};
use rusqlite::{
    types::{FromSql, FromSqlError},
    ToSql,
};
use serde::{Deserialize, Serialize};

use crate::ID_WORKER;

#[derive(Serialize, Deserialize, Debug)]
pub enum Sender {
    AI,
    ME,
}

impl ToSql for Sender {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Self::AI => "AI".to_sql(),
            Self::ME => "ME".to_sql(),
        }
    }
}

impl FromSql for Sender {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            "AI" => Ok(Sender::AI),
            "ME" => Ok(Sender::ME),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl Display for Sender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AI => write!(f, "AI"),
            Self::ME => write!(f, "ME"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(with = "super::big_number_serializer")]
    pub id: i64,
    pub chat_id: i64,
    pub sender: Sender,
    pub content: String,
    #[serde(with = "super::datetime_format")]
    pub create_time: NaiveDateTime,
}

impl Message {
    pub fn me() -> Self {
        Self {
            id: ID_WORKER.write().get_id(),
            chat_id: 0,
            sender: Sender::ME,
            content: "".to_string(),
            create_time: Local::now().naive_local(),
        }
    }

    pub fn ai() -> Self {
        Self {
            id: ID_WORKER.write().get_id(),
            chat_id: 0,
            sender: Sender::AI,
            content: "".to_string(),
            create_time: Local::now().naive_local(),
        }
    }
}
