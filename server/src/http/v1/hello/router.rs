use crate::commands;
use crate::queue;
use axum::extract::{Extension, Path};
use serde::Deserialize;

pub fn router() -> axum::Router {
    let router = axum::Router::new().route("/:name", axum::routing::get(http_hello_handler));

    return router;
}

#[derive(Deserialize, Debug)]
struct HelloParams {
    name: String,
}

async fn http_hello_handler(
    Path(params): Path<HelloParams>,
    Extension(queue_client): Extension<rumqttc::AsyncClient>,
) -> String {
    // TODO: remove test publish
    let options = commands::ping::ping::Options {
        hostname: "google.com".to_string(),
        packets: 4,
    };
    let command = commands::exec::CommandRequest {
        command: String::from("ping"),
        id: String::from("123"),
        options,
    };
    queue::connection::publish(&queue_client, "uk/command/request".to_string(), command).await;

    return format!("hello {}!", params.name);
}
