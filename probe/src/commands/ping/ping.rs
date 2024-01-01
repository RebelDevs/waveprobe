#[derive(Debug)]
pub struct Result {
    pub posix: String,
}

pub fn run() -> Result {
    let cmd_result = execute_ping();
    return Result {
        posix: cmd_result,
    };
}

pub fn execute_ping() -> String {
    return super::super::utils::posix::run("ping".to_string(), "some args".to_string());
}
