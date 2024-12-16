use std::io::Error;
use core::command::Command;

pub struct ListenService {}

impl ListenService {
    pub fn run(command: &Command) -> Result<String, Error> {
        Result::Ok("Listening file transfer...".to_string())
    }
}
