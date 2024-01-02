use regex::Regex;

#[derive(Debug, Default)]
pub struct Line {
    pub hostname: Option<String>,
    pub address: Option<String>,
    pub ttl: u32,
    pub time: f32,
}

pub fn extract(data: &str) -> Vec<Line> {
    let mut line_vec: Vec<Line> = Vec::with_capacity(data.lines().count());

    for line_data in data.lines() {
        if let Some(line) = extract_line(line_data) {
            line_vec.push(line);
        }
    }

    return line_vec;
}

fn extract_line(data: &str) -> Option<Line> {
    let re = Regex::new(r"^\d+ bytes from (?P<host>.*) \((?P<addr>.+?)\): (?:icmp_)?seq=\d+ ttl=(?P<ttl>\d+) time=(?P<time>\d*(?:\.\d+)?) ms").unwrap();

    return re.captures(data).map(|x| {
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

        return line;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let data = "64 bytes from 104.111.85.20 (104.111.85.20): icmp_seq=3 ttl=54 time=4.78 ms";
        let result = extract_line(data);

        assert!(result.is_some());
        let line = result.unwrap();
        assert_eq!(line.address.unwrap(), "104.111.85.20".to_string());
        assert_eq!(line.hostname.unwrap(), "104.111.85.20".to_string());
    }

    #[test]
    fn no_address() {
        let data = "64 bytes from 104.111.85.20 (): icmp_seq=3 ttl=54 time=4.78 ms";
        let result = extract_line(data);

        assert!(!result.is_some());
    }
}
