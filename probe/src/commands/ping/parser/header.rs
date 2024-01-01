use regex::Regex;

#[derive(Debug)]
#[derive(Default)]
pub struct Header {
    hostname: Option<String>,
    address: Option<String>,
}

pub fn extract(data: &str) -> Header {
    let mut header = Header::default();
    let re = Regex::new(r"^PING\s(?P<host>.*?)\s\((?P<addr>.+?)\)").unwrap();

    if let Some(matched_result) = re.captures(data) {
        header.hostname = matched_result.name("host").map(|s| s.as_str().to_string());
        header.address = matched_result.name("addr").map(|s| s.as_str().to_string());
    }

    return header;
}
