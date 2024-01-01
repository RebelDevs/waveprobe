use regex::Regex;

#[derive(Debug)]
pub struct Header {
    hostname: Option<String>,
    address: Option<String>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            hostname: None,
            address: None,
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub hostname: Option<String>,
    pub address: Option<String>,
    pub ttl: u32,
    pub time: f32,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            hostname: None,
            address: None,
            ttl: 0,
            time: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct RTT {
    min: f32,
    avg: f32,
    max: f32,
    mdev: f32,
}

impl Default for RTT {
    fn default() -> Self {
        Self {
            min: 0.0,
            avg: 0.0,
            max: 0.0,
            mdev: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Packets {
    total: i8,
    loss: i8,
    rcv: i8,
    drop: i8,
}

impl Default for Packets {
    fn default() -> Self {
        Self {
            total: 0,
            loss: 0,
            rcv: 0,
            drop: 0,
        }
    }
}

#[derive(Debug)]
pub struct Values {
    pub header: Header,
    pub lines: Vec<Line>,
    pub rtt: RTT,
    pub packets: Packets,
}

#[derive(Debug)]
pub struct Result {
    pub posix: String,
    pub values: Values,
}

pub fn run() -> Result {
    let cmd_result = execute_ping();
    let parse_values = parse(&cmd_result);

    return Result {
        posix: cmd_result,
        values: parse_values,
    };
}

fn execute_ping() -> String {
    return super::super::utils::posix::run("ping".to_string(), "some args".to_string());
}

fn parse(data: &str) -> Values {
    let rtt = extract_rtt(data);
    let packets = extract_packets(data);
    let header = extract_header(data);
    let lines = extract_lines(data);
    return Values {
        header,
        lines,
        rtt,
        packets,
    };
}

fn extract_rtt(data: &str) -> RTT {
    let mut rtt = RTT::default();
    let re =
        Regex::new(r"rtt min/avg/max/mdev = ([\d\.]+)/([\d\.]+)/([\d\.]+)/([\d\.]+) ms").unwrap();

    re.captures(data).map(|x| {
        match x[1].parse::<f32>() {
            Ok(num) => rtt.min = num,
            Err(e) => println!("Failed to convert rtt min, {}", e),
        }
        match x[2].parse::<f32>() {
            Ok(num) => rtt.avg = num,
            Err(e) => println!("Failed to convert rtt avg, {}", e),
        }
        match x[3].parse::<f32>() {
            Ok(num) => rtt.max = num,
            Err(e) => println!("Failed to convert rtt max, {}", e),
        }
        match x[4].parse::<f32>() {
            Ok(num) => rtt.mdev = num,
            Err(e) => println!("Failed to convert rtt mdev, {}", e),
        }
    });

    return rtt;
}

fn extract_packets(data: &str) -> Packets {
    let mut packets = Packets::default();
    let re = Regex::new(r"(\d+) packets transmitted, (\d+) received, (\d+)% packet loss").unwrap();
    re.captures(data).map(|x| {
        match x[1].parse::<i8>() {
            Ok(num) => packets.total = num,
            Err(e) => println!("Failed to convert packets total, {}", e),
        }
        match x[2].parse::<i8>() {
            Ok(num) => packets.rcv = num,
            Err(e) => println!("Failed to convert packets received, {}", e),
        }
        match x[3].parse::<i8>() {
            Ok(num) => packets.loss = num,
            Err(e) => println!("Failed to convert packets loss, {}", e),
        }
    });

    packets.drop = packets.total - packets.rcv;

    return packets;
}

fn extract_header(data: &str) -> Header {
    let mut header = Header::default();
    let re = Regex::new(r"^PING\s(?P<host>.*?)\s\((?P<addr>.+?)\)").unwrap();

    if let Some(matched_result) = re.captures(data) {
        header.hostname = matched_result.name("host").map(|s| s.as_str().to_string());
        header.address = matched_result.name("addr").map(|s| s.as_str().to_string());
    }

    return header;
}

fn extract_lines(data: &str) -> Vec<Line> {
    let mut line_vec: Vec<Line> = Vec::with_capacity(data.lines().count());
    let re = Regex::new(r"^\d+ bytes from (?P<host>.*) ((?P<addr>.+?)): (?:icmp_)?seq=\d+ ttl=(?P<ttl>\d+) time=(?P<time>\d*(?:\.\d+)?) ms").unwrap();

    for line_data in data.lines() {
        re.captures(line_data).map(|x| {
            let mut line = Line::default();

            line.hostname = x.name("host").map(|s| s.as_str().to_string());
            line.address = x.name("addr").map(|s| s.as_str().to_string());

            if let Some(ttl) = x.name("ttl") {
                match ttl.as_str().parse::<u32>() {
                    Ok(num) => line.ttl = num,
                    Err(e) => println!("Failed to convert line ttl, {}", e),
                }
            }

            if let Some(time) = x.name("time") {
                match time.as_str().parse::<f32>() {
                    Ok(num) => line.time = num,
                    Err(e) => println!("Failed to convert line time, {}", e),
                }
            }

            line_vec.push(line);
        });
    }

    return line_vec;
}
