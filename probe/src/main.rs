#[allow(clippy::needless_return)]

mod commands;

fn main() {
    let options = commands::ping::ping::Options {
        hostname: "google.com".to_string(),
        packets: 4,
    };

    let result = commands::ping::ping::run(options);
    println!("{:#?}", result);
}
