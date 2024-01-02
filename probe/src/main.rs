#![allow(clippy::needless_return)]

extern crate dotenv;
use dotenv::dotenv;

mod commands;

fn main() {
    dotenv().ok();

    let options = commands::ping::ping::Options {
        hostname: "google.com".to_string(),
        packets: 4,
    };

    let result = commands::ping::ping::run(options);
    println!("{:#?}", result);
}
