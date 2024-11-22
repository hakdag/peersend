use std::io::Error;

#[derive(Debug)]
pub struct HelpService {}

impl HelpService {
    pub fn run() -> Result<String, Error> {
        Result::Ok("No help is supported at the moment.".to_string())
    }
}