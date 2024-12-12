use crate::command::{Command, CommandType, CommandArgument};
use std::io::Error;

use super::get_arg;

pub struct RegisterDeviceParser {}

impl RegisterDeviceParser {
    pub fn parse(args: &Vec<String>) -> Result<Command, Error> {
        let count = args.len();
        if count != 3 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Register Device is expecting parameters: devicename"));
        }

        let arguments = Vec::from([
            CommandArgument::new(get_arg(args, 2).to_string()), // devicename
        ]);
        let command: Command = Command::new("Register Device".to_string(), CommandType::RegisterDevice, Some(arguments));
        Result::Ok(command)
    }
}