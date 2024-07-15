use std::str::FromStr;

use regex::Regex;

mod handler;
pub use handler::handle;

pub const SUB_NAME: &str = "+/command/check/ack";
pub fn is_match(topic: &str) -> bool {
    let re = Regex::new(r"^(.*)/command/check/ack$").unwrap();
    return re.is_match(topic);
}

pub fn extract_id(topic: &str) -> i32 {
    let re = Regex::new(r"^(.*)/command/check/ack$").unwrap();
    let request_id = re
        .captures(&topic)
        .and_then(|x| x.get(1))
        .map(|x| x.as_str())
        .unwrap_or("unknown");

    return FromStr::from_str(request_id).unwrap();
}
