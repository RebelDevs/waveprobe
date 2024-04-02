use crate::commands;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct MeasurementRow {
    pub id: i64,
    pub status: String,
    pub command: String,
    pub parameters: serde_json::Value,
    pub location: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pending,
    Processing,
    Completed,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OptionsEnum {
    Ping(commands::ping::ping::Options),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Measurement {
    pub id: i64,
    pub status: Status,
    pub command: String,
    pub parameters: Option<OptionsEnum>,
    pub location: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeasurementCreate {
    pub status: Status,
    pub command: String,
    pub parameters: serde_json::Value,
    pub location: String,
}
