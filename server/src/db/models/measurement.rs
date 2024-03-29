use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, FromRow)]
pub struct Measurement {
    pub id: i64,
    pub command: String,
    pub parameters: String,
    pub location: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct MeasurementCreate {
    pub command: String,
    pub parameters: String,
    pub location: String,
}
