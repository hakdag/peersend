use crate::command::{Command, CommandType};
use std::env;
use std::io::Error;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    pub fn parse() -> Result<Command, Error> {
        let args: Vec<String> = env::args().collect();
        let count = args.len();
        if count < 2 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "not enough arguments"));
        }

        // currently supporting single command
        let arg = Self::get_current_arg(&args, 1);
        match arg {
            "help" => return Result::Ok(Command::new("Help".to_string(), CommandType::Help)),
            "version" => return Result::Ok(Command::new("Version".to_string(), CommandType::Version)),
            "create-user" => return Result::Ok(Command::new("Create User".to_string(), CommandType::CreateUser)),
            "login" => return Result::Ok(Command::new("Login".to_string(), CommandType::Login)),
            "register" => return Result::Ok(Command::new("Register Device".to_string(), CommandType::RegisterDevice)),
            "send" => return Result::Ok(Command::new("Send File".to_string(), CommandType::Send)),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "unknown command"))
        }
    }

    fn get_current_arg(args: &Vec<String>, index: usize) -> &str {
        match args.get(index) {
            Some(arg) => arg,
            None => ""
        }
    }
}
