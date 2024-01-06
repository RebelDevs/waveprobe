use std::process::Output;
use tokio::process::Command;

pub async fn run(script: String, args: Vec<String>) -> Result<String, String> {
    let process = execute(script, args).await;

    let stdres = if process.status.success() {
        process.stdout
    } else {
        process.stderr
    };

    match std::str::from_utf8(&stdres) {
        Ok(str) => {
            if process.status.success() {
                Ok(str.trim().to_string())
            } else {
                Err(str.trim().to_string())
            }
        }
        Err(e) => {
            eprintln!("failed to parse result, {}", e);
            Err("Internal error".to_string())
        }
    }
}

async fn execute(script: String, args: Vec<String>) -> Output {
    Command::new(script)
        .args(args)
        .output()
        .await
        .expect("failed to parse result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let result = run(
            "./bin/mocks/ping.sh".to_string(),
            vec!["-c 4".to_string(), "success".to_string()],
        );

        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("PING"));
    }

    #[test]
    fn fail() {
        let result = run(
            "./bin/mocks/ping.sh".to_string(),
            vec!["-c 4".to_string(), "error".to_string()],
        );

        assert_eq!(result, Err("error".to_string()));
    }
}
