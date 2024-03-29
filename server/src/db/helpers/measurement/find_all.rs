use crate::db::models::measurement::Measurement;

fn construct_query() -> String {
    return format!("SELECT * FROM measurements");
}

pub async fn find_all(pool: &sqlx::SqlitePool) -> Vec<Measurement> {
    let measurement = sqlx::query_as::<_, Measurement>(&construct_query())
        .fetch_all(pool)
        .await;

    if let Err(e) = measurement {
        eprintln!("Error: {}", e);
        return Vec::new();
    }

    return measurement.unwrap();
}
