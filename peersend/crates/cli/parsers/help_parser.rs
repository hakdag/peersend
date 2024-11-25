use crate::command::{Command, CommandType};
use std::io::Error;

pub struct HelpParser {}

impl HelpParser {
    pub fn parse() -> Result<Command, Error> {
        Result::Ok(Command::new("Help".to_string(), CommandType::Help, None))
    }
}