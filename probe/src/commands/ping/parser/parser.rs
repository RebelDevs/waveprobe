use super::header;
use super::lines;
use super::stats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Values {
    pub header: header::Header,
    pub lines: Vec<lines::Line>,
    pub rtt: stats::RTT,
    pub packets: stats::Packets,
}

pub fn parse(data: &str) -> Values {
    let (rtt, packets) = stats::extract(data);
    return Values {
        header: header::extract(data),
        lines: lines::extract(data),
        rtt,
        packets,
    };
}
