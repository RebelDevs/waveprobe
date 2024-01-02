use std::process::Command;

pub fn run(script: String, args: Vec<String>) -> Result<String, String> {
    let process = Command::new(script)
        .args(args)
        .output()
        .expect("failed to start command");

    let stdres = if process.status.success() {
        process.stdout
    } else {
        process.stderr
    };

    match std::str::from_utf8(&stdres) {
        Ok(str) => {
            if process.status.success() {
                Ok(str.to_string())
            } else {
                Err(str.to_string())
            }
        }
        Err(e) => {
            eprintln!("failed to parse result, {}", e);
            Err("Internal error".to_string())
        }
    }
}
