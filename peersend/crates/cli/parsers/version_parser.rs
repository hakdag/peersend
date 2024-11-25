use crate::command::{Command, CommandType};
use std::io::Error;

pub struct VersionParser {}

impl VersionParser {
    pub fn parse() -> Result<Command, Error> {
        Result::Ok(Command::new("Version".to_string(), CommandType::Version, None))
    }
}