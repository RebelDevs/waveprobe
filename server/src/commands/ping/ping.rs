use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub hostname: String,
    pub packets: u8,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PingResult {
    pub posix: String,
    pub values: Values,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Values {
    pub header: Header,
    pub lines: Vec<Line>,
    pub rtt: RTT,
    pub packets: Packets,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Header {
    pub hostname: Option<String>,
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RTT {
    pub min: f32,
    pub avg: f32,
    pub max: f32,
    pub mdev: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Packets {
    pub total: i8,
    pub loss: i8,
    pub rcv: i8,
    pub drop: i8,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Line {
    pub hostname: Option<String>,
    pub address: Option<String>,
    pub ttl: u32,
    pub time: f32,
}
