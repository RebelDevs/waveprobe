#![allow(clippy::needless_return)]

mod commands;
mod queue;

extern crate dotenv;
use dotenv::dotenv;
use tokio::task;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let queue_init = task::spawn(async {
        queue::connection::init().await;
    });

    match queue_init.await {
        Ok(_) => println!("mqtt down"),
        Err(e) => eprintln!("{}", e),
    }
}
