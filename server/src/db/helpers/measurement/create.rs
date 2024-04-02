use super::utils;
use crate::db::models::measurement::{Measurement, MeasurementCreate, MeasurementRow};
use chrono;

pub async fn create(
    data: MeasurementCreate,
    pool: &sqlx::SqlitePool,
) -> Result<Measurement, sqlx::Error> {
    let date = chrono::Utc::now().to_rfc2822();
    let status = serde_json::to_string(&data.status).unwrap();

    let row = sqlx::query_as::<_, MeasurementRow>(
        "INSERT INTO measurements (
            status,
            command,
            location,
            parameters,
            updated_at,
            created_at
        ) VALUES (?, ?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(status)
    .bind(data.command)
    .bind(data.location)
    .bind(data.parameters)
    .bind(&date)
    .bind(&date)
    .fetch_one(pool)
    .await;

    if let Err(e) = row {
        eprintln!("Error: {}", e);
        return Err(e);
    }

    return Ok(utils::row_to_measurement(row?));
}
