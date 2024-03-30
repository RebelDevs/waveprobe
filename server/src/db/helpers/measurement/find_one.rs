use super::utils;
use crate::db::models::measurement::{Measurement, MeasurementRow};

fn construct_query(id: i32) -> String {
    return format!("SELECT * FROM measurements WHERE id = {} limit 1", id);
}

pub async fn find_one(id: i32, pool: &sqlx::SqlitePool) -> Option<Measurement> {
    let row = sqlx::query_as::<_, MeasurementRow>(&construct_query(id))
        .fetch_one(pool)
        .await;

    if let Err(e) = row {
        eprintln!("Error: {}", e);
        return None;
    }

    return Some(utils::row_to_measurement(row.unwrap()));
}
