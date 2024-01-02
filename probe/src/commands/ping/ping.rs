use super::parser;

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
    let args = vec![
        "-4".to_string(),
        format!("-c {}", options.packets),
        "-i 0.2".to_string(),
        "-w 15".to_string(),
        options.hostname,
    ];

    super::super::utils::posix::run("ping".to_string(), args)
}
