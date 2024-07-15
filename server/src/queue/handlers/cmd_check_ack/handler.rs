use crate::commands;
use crate::db;
use crate::queue;
use rumqttc::AsyncClient;

pub async fn handle(
    client: &AsyncClient,
    db_pool: &sqlx::SqlitePool,
    request_id: i32,
    _payload: &[u8],
) {
    let request = db::helpers::measurement::find_one(request_id, db_pool).await;
    if !request.is_some() {
        println!("request not found, {}", request_id);
        return;
    }

    let data = request.unwrap();

    let command = commands::exec::CommandRequest {
        command: data.command,
        id: data.id.to_string(),
        options: data.parameters,
    };

    queue::connection::publish(&client, "uk/command/request".to_string(), command).await;
}
