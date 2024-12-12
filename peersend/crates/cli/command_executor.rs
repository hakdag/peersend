use core::command::{Command, CommandType};
use std::io::Error;
use services::file::FileStorage;
use services::help::HelpService;
use services::version::VersionService;
use services::create_user::CreateUserService;
use services::login::LoginService;
use services::register_device::RegisterDeviceService;
use services::send_file::SendFileService;
use comms::redis_communication::RedisCommunication;

pub struct CommandExecutor {}

impl CommandExecutor {
    pub fn execute(command: &Command) -> Result<String, Error> {
        println!("Executing command: {}", command.name);
        let rc = match RedisCommunication::new() {
            Ok(rc) => rc,
            Err(e) => return Err(e),
        };
        let fs = FileStorage {};
        match command.command_type {
            CommandType::Help => HelpService::run(),
            CommandType::Version => VersionService::run(),
            CommandType::CreateUser => CreateUserService::run(command),
            CommandType::Login => {
                let ls: LoginService<RedisCommunication, FileStorage> = LoginService::new(rc, fs);
                ls.run(command)
            },
            CommandType::RegisterDevice => {
                let register_device_service: RegisterDeviceService<RedisCommunication, FileStorage> = RegisterDeviceService::new(rc, fs);
                register_device_service.run(command)
            },
            CommandType::Send => {
                let send_file_service: SendFileService<RedisCommunication, FileStorage> = SendFileService::new(rc, fs);
                send_file_service.run(command)
            },
        }
    }
}
