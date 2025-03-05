use crate::command::{Command, CommandType, CommandArgument};
use std::io::Error;

use super::get_arg;

pub struct CreateUserParser {}

impl CreateUserParser {
    pub fn parse(args: &Vec<String>) -> Result<Command, Error> {
        let count = args.len();
        if count != 5 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Create User is expecting parameters: username email password"));
        }

        let arguments = Vec::from([
            CommandArgument::new(get_arg(args, 2).to_string()), // username
            CommandArgument::new(get_arg(args, 3).to_string()), // password
            CommandArgument::new(get_arg(args, 4).to_string()), // email
        ]);
        let command: Command = Command::new("Create User".to_string(), CommandType::CreateUser, Some(arguments));
        Result::Ok(command)
    }
}