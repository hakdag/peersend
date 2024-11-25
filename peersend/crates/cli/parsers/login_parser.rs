use crate::command::{Command, CommandType, CommandArgument};
use std::io::Error;

use super::get_arg;

pub struct LoginParser {}

impl LoginParser {
    pub fn parse(args: &Vec<String>) -> Result<Command, Error> {
        let count = args.len();
        if count != 4 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Login is expecting parameters: username, password"));
        }

        let arguments = Vec::from([
            CommandArgument::new(get_arg(args, 2).to_string()), // username
            CommandArgument::new(get_arg(args, 3).to_string()), // password
        ]);
        let command: Command = Command::new("Login".to_string(), CommandType::Login, Some(arguments));
        Result::Ok(command)
    }
}