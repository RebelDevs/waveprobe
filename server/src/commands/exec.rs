use super::ping;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommandRequest<T> {
    pub command: String,
    pub id: String,
    pub options: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandResultEnum {
    Ping(ping::ping::PingResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandInstance {
    pub id: String,
    pub result: CommandResultEnum,
}
