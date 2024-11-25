use crate::command::{Command, CommandType, CommandArgument};
use std::io::Error;

use super::get_arg;

pub struct SendFileParser {}

impl SendFileParser {
    pub fn parse(args: &Vec<String>) -> Result<Command, Error> {
        let count = args.len();
        if count != 5 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Send File is expecting parameters: filepath, source device name, target device name"));
        }

        let arguments = Vec::from([
            CommandArgument::new(get_arg(args, 2).to_string()), // filepath
            CommandArgument::new(get_arg(args, 3).to_string()), // source device name
            CommandArgument::new(get_arg(args, 4).to_string()), // target device name
        ]);
        let command: Command = Command::new("Send File".to_string(), CommandType::Send, Some(arguments));
        Result::Ok(command)
    }
}