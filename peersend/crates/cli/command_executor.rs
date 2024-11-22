use core::command::{Command, CommandType};
use std::io::Error;
use services::help::HelpService;
use services::version::VersionService;
use services::create_user::CreateUserService;
use services::login::LoginService;
use services::register_device::RegisterDeviceService;
use services::send_file::SendFileService;

#[derive(Debug)]
pub struct CommandExecutor {}

impl CommandExecutor {
    pub fn execute(command: &Command) -> Result<String, Error> {
        println!("Executing command: {}", command.name);
        match command.command_type {
            CommandType::Help => HelpService::run(),
            CommandType::Version => VersionService::run(),
            CommandType::CreateUser => CreateUserService::run(command),
            CommandType::Login => LoginService::run(command),
            CommandType::RegisterDevice => RegisterDeviceService::run(command),
            CommandType::Send => SendFileService::run(command),
        }
    }
}
