use crate::commands::exec::{CommandInstance, CommandResultEnum};
use serde_json::{self, Value};

fn parse(data: &[u8]) -> Result<CommandInstance, String> {
    let payload = match serde_json::from_slice::<Value>(&data) {
        Ok(payload) => payload,
        Err(e) => {
            eprintln!("from_slice, {}", e.to_string());
            return Err("from_slice failed".to_string());
        }
    };

    let result = match serde_json::from_value::<CommandInstance>(payload) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("from_value, {}", e.to_string());
            return Err("from_value failed".to_string());
        }
    };

    return Ok(result);
}

pub fn handle(data: &[u8]) -> Result<(), String> {
    let result = match parse(data) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("failed to parse the response, {}", e);
            return Err("response parse failed".to_string());
        }
    };

    match result.result {
        CommandResultEnum::Ping(ping) => {
            println!("{:#?}", ping);
        }
    };

    return Ok(());
}
