use regex::Regex;

#[derive(Debug, Default)]
pub struct Header {
    pub hostname: Option<String>,
    pub address: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_suceed() {
        let data = "PING elocast.com (123.123.123.123) 56(84) bytes of data.";
        let result = extract(data);

        assert_eq!(result.hostname, Some("elocast.com".to_string()));
        assert_eq!(result.address, Some("123.123.123.123".to_string()));
    }

    #[test]
    fn no_ip() {
        let data = "PING elocast.com () 56(84) bytes of data.";
        let result = extract(data);

        println!("{:#?}", result);

        assert_eq!(result.hostname, Some("elocast.com".to_string()));
        assert_eq!(result.address, None);
    }
}
