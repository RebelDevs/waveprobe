use super::super::super::super::commands;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct CommandRequest<T> {
    pub command: String,
    pub id: String,
    pub options: T,
}

#[derive(Debug)]
pub enum CommandResultEnum {
    Ping(commands::ping::ping::PingResult),
}

#[derive(Debug)]
pub struct CommandInstance {
    pub id: String,
    pub result: CommandResultEnum,
}

pub async fn handle(payload: &[u8]) -> Result<CommandInstance, String> {
    let payload_serde: Result<Value, serde_json::Error> = serde_json::from_slice(payload);

    if let Err(e) = payload_serde {
        println!("parse error, {}", e);
        return Err(String::from("parse_error"));
    }

    let payload_json = payload_serde.unwrap();

    match payload_json.get("command").and_then(Value::as_str) {
        Some("ping") => {
            let request: Result<CommandRequest<commands::ping::ping::Options>, serde_json::Error> =
                serde_json::from_value(payload_json);

            if let Err(e) = request {
                println!("parse error, {}", e);
                return Err(String::from("parse_error"));
            }

            let request_json = request.unwrap();

            let command = commands::ping::ping::run(request_json.options);
            let instance = CommandInstance {
                id: request_json.id,
                result: CommandResultEnum::Ping(command),
            };

            Ok(instance)
        }
        _ => {
            println!("Unknown command");
            Err("unknown_command".to_string())
        }
    }
}
