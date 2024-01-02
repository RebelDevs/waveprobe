use super::parser;
use std::env;

pub struct Options {
    pub hostname: String,
    pub packets: u8,
}

#[derive(Debug, Default)]
pub struct PingResult {
    pub posix: String,
    pub values: parser::Values,
}

pub fn run(options: Options) -> PingResult {
    let mut result = PingResult::default();
    let posix_result = execute_ping(options);

    match posix_result {
        Ok(str) => {
            result.posix = str.clone();
            result.values = parser::parse(&str);
        }
        Err(e) => {
            result.posix = e;
        }
    }

    return result;
}

fn execute_ping(options: Options) -> Result<String, String> {
    let script_path = get_script_path();

    let args = vec![
        "-4".to_string(),
        format!("-c {}", options.packets),
        "-i 0.2".to_string(),
        "-w 15".to_string(),
        options.hostname,
    ];

    super::super::utils::posix::run(script_path.to_string(), args)
}

fn get_script_path() -> String {
    env::var("PING_PATH").map_or("/usr/bin/ping".to_string(), |x| x)
}

#[cfg(test)]
mod execute_ping_tests {
    use super::*;

    #[test]
    fn success() {
        let options = Options {
            hostname: "success".to_string(),
            packets: 4,
        };

        let result = execute_ping(options);

        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("PING"));
    }

    #[test]
    fn error() {
        let options = Options {
            hostname: "error".to_string(),
            packets: 4,
        };

        let result = execute_ping(options);

        assert!(result.is_err());
        assert_eq!(result, Err("error".to_string()));
    }
}

#[cfg(test)]
mod run_tests {
    use super::*;

    #[test]
    fn success() {
        let options = Options {
            hostname: "success".to_string(),
            packets: 3,
        };

        let result = run(options);

        assert!(result.posix.starts_with("PING"));
        assert_eq!(result.values.header.hostname, Some("success".to_string()));
        assert_eq!(result.values.header.address, Some("1.2.3.4".to_string()));
        assert_eq!(result.values.packets.total, 3);
        assert_eq!(result.values.rtt.min, 3.339);
    }

    #[test]
    fn error() {
        let options = Options {
            hostname: "error".to_string(),
            packets: 4,
        };

        let result = run(options);

        assert!(result.posix.starts_with("error"));
        assert_eq!(result.values.header.hostname, None);
        assert_eq!(result.values.header.address, None);
        assert_eq!(result.values.packets.total, 0);
        assert_eq!(result.values.rtt.min, 0.0);
    }
}
