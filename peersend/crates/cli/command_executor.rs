use core::command::{Command, CommandType};
use std::io::Error;
use services::help::HelpService;
use services::version::VersionService;

#[derive(Debug)]
pub struct CommandExecutor {}

impl CommandExecutor {
    pub fn execute(command: &Command) -> Result<String, Error> {
        println!("Executing command: {}", command.name);
        match command.command_type {
            CommandType::Help => HelpService::run(),
            CommandType::Version => VersionService::run(),
        }
    }
}
