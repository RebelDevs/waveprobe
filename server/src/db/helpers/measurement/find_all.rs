use crate::db::models::measurement::{Measurement, MeasurementRow};

fn construct_query() -> String {
    return format!("SELECT * FROM measurements");
}

pub async fn find_all(pool: &sqlx::SqlitePool) -> Vec<Measurement> {
    let row = sqlx::query_as::<_, MeasurementRow>(&construct_query())
        .fetch_all(pool)
        .await;

    if let Err(e) = row {
        eprintln!("Error: {}", e);
        return Vec::new();
    }

    let measurements = row
        .unwrap()
        .iter()
        .map(|row| {
            return super::utils::row_to_measurement(row.clone());
        })
        .collect();

    return measurements;
}
