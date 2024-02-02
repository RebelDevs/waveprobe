use regex::Regex;

mod handler;
pub use handler::handle;

pub const SUB_NAME: &str = "+/command/response";
pub fn is_match(name: &str) -> bool {
    let re = Regex::new(r"^(.*)/command/response$").unwrap();
    return re.is_match(name);
}
