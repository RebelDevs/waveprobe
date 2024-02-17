#![allow(clippy::needless_return)]

mod commands;
mod queue;

extern crate dotenv;

use dotenv::dotenv;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _client = queue::connection::init().await;

    loop {}
}
