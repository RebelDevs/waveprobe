#[allow(clippy::needless_return)]

mod commands;

fn main() {
    let result = commands::ping::ping::run();
    println!("{:#?}", result);
}
