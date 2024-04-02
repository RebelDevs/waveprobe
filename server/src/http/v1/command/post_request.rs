use crate::db;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    Extension(db_pool): Extension<sqlx::SqlitePool>,
    JsonBody(body): JsonBody<RequestBody>,
) -> Result<types::ApiResponse<ResponseBody>, types::HttpError> {
    let measurement = db::models::measurement::MeasurementCreate {
        status: db::models::measurement::Status::Pending,
        command: body.command.clone(),
        parameters: serde_json::to_value(body.options).unwrap(),
        location: body.location.clone(),
    };

    let db_record = db::helpers::measurement::create(measurement, &db_pool).await;
    if db_record.is_err() {
        return Err(types::HttpError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            headers: None,
            body: types::ErrorResponseBody {
                error_code: "db_error".to_string(),
                message: "Failed to create record".to_string(),
            },
        });
    }
    let id = db_record.map(|record| record.id.to_string()).unwrap();

    let topic = format!("{}/command/check", body.location);
    let command = commands::exec::CommandCheck {
        command: body.command,
        request_id: id.clone(),
    };

    queue::connection::publish(&queue_client, topic, command).await;

    return Ok((
        axum::http::StatusCode::CREATED,
        axum::http::HeaderMap::new(),
        Json(ResponseBody { id }),
    ));
}
