use std::io::Error;

#[derive(Debug)]
pub struct VersionService {}

impl VersionService {
    pub fn run() -> Result<String, Error> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        Result::Ok(String::from("v".to_string() + VERSION))
    }
}