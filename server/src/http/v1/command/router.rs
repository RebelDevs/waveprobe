use std::sync::Arc;

use crate::commands;
use crate::http::types;
use crate::queue;
use axum::extract::{Extension, Json, Path};
use serde::{Deserialize, Serialize};

pub fn router() -> axum::Router {
    let router = axum::Router::new()
        .route("/:id", axum::routing::get(get_result))
        .route("/request", axum::routing::post(post_request));

    return router;
}

#[derive(Deserialize, Debug)]
struct GetResultParams {
    id: String,
}

async fn get_result(Path(params): Path<GetResultParams>) -> types::ApiResponse<ResponseBody> {
    let headers = {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers
    };

    return (
        axum::http::StatusCode::OK,
        headers,
        Json(ResponseBody { id: params.id }),
    );
}

#[derive(Deserialize, Debug)]
struct RequestBody {
    command: String,
    options: commands::ping::ping::Options,
}

#[derive(Serialize, Debug)]
struct ResponseBody {
    pub id: String,
}

async fn post_request(
    Extension(queue_client): Extension<Arc<rumqttc::AsyncClient>>,
    Json(body): Json<RequestBody>,
) -> Json<ResponseBody> {
    let id = String::from("123");

    let command = commands::exec::CommandRequest {
        command: body.command,
        id: id.clone(),
        options: body.options,
    };
    queue::connection::publish(&queue_client, "uk/command/request".to_string(), command).await;

    return Json(ResponseBody { id });
}
