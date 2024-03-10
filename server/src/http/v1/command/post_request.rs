use crate::http::types;
use crate::queue;
use crate::{commands, http::v1::utils::body_parser::JsonBody};
use axum::extract::{Extension, Json};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct RequestBody {
    location: String,
    limit: i16,
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

        let limit = v
            .get("limit")
            .and_then(serde_json::Value::as_i64)
            .map(|val| val as i16)
            .unwrap_or(10);

        let location = v
            .get("location")
            .and_then(serde_json::Value::as_str)
            .map(|s| s.to_owned())
            .unwrap_or("*".to_owned());

        return Ok(RequestBody {
            command: command.to_owned(),
            limit,
            location,
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
    // TODO: replace with db call
    let id = String::from("123");

    let topic = format!("{}/command/check", body.location);
    let command = commands::exec::CommandCheck {
        command: body.command,
        request_id: id.clone(),
    };

    queue::connection::publish(&queue_client, topic, command).await;

    return (
        axum::http::StatusCode::CREATED,
        axum::http::HeaderMap::new(),
        Json(ResponseBody { id }),
    );
}
