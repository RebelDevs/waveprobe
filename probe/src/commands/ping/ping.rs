use std::process::Output;

use super::parser::{parse, Values};

pub struct Options {
    pub hostname: String,
    pub packets: u8,
}

#[derive(Debug, Default)]
pub struct Result {
    pub posix: String,
    pub values: Values,
}

pub fn run(options: Options) -> Result {
    let mut result = Result::default();
    let posix_result = execute_ping(options);

    let stdres = if posix_result.status.success() {
        posix_result.stdout
    } else {
        posix_result.stderr
    };

    match std::str::from_utf8(&stdres) {
        Ok(str) => {
            result.posix = str.to_string();

            if posix_result.status.success() {
                result.values = parse(str);
            }
        }
        Err(e) => {
            println!("failed to parse result, {}", e);
            result.posix = "Internal error".to_string();
        }
    }

    return result;
}

fn execute_ping(options: Options) -> Output {
    let args = vec![
        "-4".to_string(),
        format!("-c {}", options.packets),
        "-i 0.2".to_string(),
        "-w 15".to_string(),
        options.hostname,
    ];

    super::super::utils::posix::run("ping".to_string(), args)
}
