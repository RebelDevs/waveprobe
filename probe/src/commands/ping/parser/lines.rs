use regex::Regex;

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

pub fn extract(data: &str) -> Vec<Line> {
    let mut line_vec: Vec<Line> = Vec::with_capacity(data.lines().count());
    let re = Regex::new(r"^\d+ bytes from (?P<host>.*) \((?P<addr>.+?)\): (?:icmp_)?seq=\d+ ttl=(?P<ttl>\d+) time=(?P<time>\d*(?:\.\d+)?) ms").unwrap();

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
