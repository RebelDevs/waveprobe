use sqlx::sqlite;

pub async fn connect() -> Result<sqlite::SqlitePool, sqlx::Error> {
    let pool = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./data/db.sqlite")
        .await?;

    return Ok(pool);
}
