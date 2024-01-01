use super::parser::{parse, Values};

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
