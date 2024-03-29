use crate::db::models::measurement::Measurement;

fn construct_query(id: i32) -> String {
    return format!("SELECT * FROM measurements WHERE id = {} limit 1", id);
}

pub async fn find_one(id: i32, pool: &sqlx::SqlitePool) -> Option<Measurement> {
    let measurement = sqlx::query_as::<_, Measurement>(&construct_query(id))
        .fetch_one(pool)
        .await;

    if let Err(e) = measurement {
        eprintln!("Error: {}", e);
        return None;
    }

    return Some(measurement.unwrap());
}
