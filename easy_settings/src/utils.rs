use lazy_static::lazy_static;
use regex::Regex;

pub fn check_if_valid_env(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[a-zA-Z_]+[a-zA-Z0-9_]*$").unwrap();
    }
    RE.is_match(text)
}
