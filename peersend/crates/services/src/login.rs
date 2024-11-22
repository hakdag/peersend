use std::io::Error;
use core::command::Command;

#[derive(Debug)]
pub struct LoginService {}

impl LoginService {
    pub fn run(command: &Command) -> Result<String, Error> {
        let msg = format!("{} {}", command.name, "not implemented!!");
        Result::Ok(msg)
    }
}