#![allow(clippy::needless_return)]

extern crate dotenv;
use dotenv::dotenv;
use tokio::task;

mod commands;
mod queue;
use queue::handlers::command_execute;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let queue_init = task::spawn(async {
        queue::connection::init().await;
    });

    task::spawn(async {
        let options = commands::ping::ping::Options {
            hostname: "google.com".to_string(),
            packets: 4,
        };
        let command = command_execute::handler::CommandRequest {
            command: String::from("ping"),
            id: String::from("123"),
            options,
        };

        match serde_json::to_string(&command) {
            Ok(json) => {
                let result = command_execute::handler::handle(json.as_bytes());
                println!("{:?}", json);
                println!("{:?}", result);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    });

    match queue_init.await {
        Ok(_) => println!("mqtt down"),
        Err(e) => eprintln!("{}", e),
    }
}
