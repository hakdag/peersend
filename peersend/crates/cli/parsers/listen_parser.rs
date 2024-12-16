use crate::command::{Command, CommandType};
use std::io::Error;

pub struct ListenParser {}

impl ListenParser {
    pub fn parse() -> Result<Command, Error> {
        let command: Command = Command::new("Listen".to_string(), CommandType::Listen, None);
        Result::Ok(command)
    }
}
