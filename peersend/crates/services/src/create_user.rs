use std::io::Error;
use core::command::Command;

#[derive(Debug)]
pub struct CreateUserService {}

impl CreateUserService {
    pub fn run(command: &Command) -> Result<String, Error> {
        let msg = format!("{} {}", command.name, "not implemented!!");
        Result::Ok(msg)
    }
}