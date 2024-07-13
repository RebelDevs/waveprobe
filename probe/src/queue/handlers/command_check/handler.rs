use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub request_id: String,
}

pub async fn handle(payload: &[u8]) -> Result<String, String> {
    let payload_serde: Result<Value, serde_json::Error> = serde_json::from_slice(payload);

    if let Err(e) = payload_serde {
        println!("parse error, {}", e);
        return Err(String::from("parse_error"));
    }

    let payload_json = payload_serde.unwrap();

    let data: Result<Payload, serde_json::Error> = serde_json::from_value(payload_json);

    if let Err(e) = data {
        println!("parse error, {}", e);
        return Err(String::from("parse_error"));
    }

    return Ok(data.unwrap().request_id);
}
