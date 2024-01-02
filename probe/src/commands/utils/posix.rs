use std::process::{Command, Output};

pub fn run(script: String, args: Vec<String>) -> Output {
    Command::new(script)
        .args(args)
        .output()
        .expect("failed to start command")
}
