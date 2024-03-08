use crate::commands;
use crate::http::types;
use crate::queue;
use axum::extract::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    command: String,
    options: commands::ping::ping::Options,
}

#[derive(Serialize, Debug)]
pub struct ResponseBody {
    pub id: String,
}

pub async fn post_request(
    Extension(queue_client): Extension<Arc<rumqttc::AsyncClient>>,
    Json(body): Json<RequestBody>,
) -> types::ApiResponse<ResponseBody> {
    let id = String::from("123");

    let command = commands::exec::CommandRequest {
        command: body.command,
        id: id.clone(),
        options: body.options,
    };
    queue::connection::publish(&queue_client, "uk/command/request".to_string(), command).await;

    return (
        axum::http::StatusCode::CREATED,
        axum::http::HeaderMap::new(),
        Json(ResponseBody { id }),
    );
}
