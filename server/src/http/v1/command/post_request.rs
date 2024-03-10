use crate::http::types;
use crate::queue;
use crate::{commands, http::v1::utils::body_parser::JsonBody};
use axum::extract::{Extension, Json};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct RequestBody {
    command: String,
    options: OptionsEnum,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum OptionsEnum {
    Ping(commands::ping::ping::Options),
}

impl<'de> Deserialize<'de> for RequestBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: serde_json::Value = serde_json::Value::deserialize(deserializer)?;

        let command = v
            .get("command")
            .and_then(|s| serde_json::Value::as_str(s).map(|s| s.to_lowercase()))
            .ok_or_else(|| serde::de::Error::missing_field("command"))?;

        let options = match command.as_str() {
            "ping" => serde_json::from_value(v.get("options").unwrap().clone())
                .map(OptionsEnum::Ping)
                .map_err(serde::de::Error::custom)?,
            _ => return Err(serde::de::Error::custom("unknown command")),
        };

        return Ok(RequestBody {
            command: command.to_owned(),
            options,
        });
    }
}

#[derive(Serialize, Debug)]
pub struct ResponseBody {
    pub id: String,
}

pub async fn post_request(
    Extension(queue_client): Extension<Arc<rumqttc::AsyncClient>>,
    JsonBody(body): JsonBody<RequestBody>,
) -> types::ApiResponse<ResponseBody> {
    // generate id
    // save measurement in db
    // publish command request
    // - whats the prefix??
    // return id

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
