use crate::db::models::measurement::{Measurement, MeasurementCreate};
use chrono;

pub async fn create(
    measurement: MeasurementCreate,
    pool: &sqlx::SqlitePool,
) -> Result<Measurement, sqlx::Error> {
    let date = chrono::Utc::now().to_rfc2822();

    let measurement = sqlx::query_as::<_, Measurement>(
        "INSERT INTO measurements (
            command,
            location,
            parameters,
            updated_at,
            created_at
        ) VALUES (?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(measurement.command)
    .bind(measurement.parameters)
    .bind(measurement.location)
    .bind(&date)
    .bind(&date)
    .fetch_one(pool)
    .await;

    if let Err(e) = measurement {
        eprintln!("Error: {}", e);
        return Err(e);
    }

    return measurement;
}
