use core::command::{Command, CommandType};
use std::io::Error;
use comms::stun_communicator::STUNCommunicator;
use comms::tcp_communicator::TCPCommunicator;
use comms::udt_communicator::UDTCommunicator;
use services::file::FileStorage;
use services::help::HelpService;
use services::listen::ListenService;
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
        let tcpc = TCPCommunicator::new();
        let udtc = UDTCommunicator::new();
        let stun_server = "stun.l.google.com:19302"; // Example STUN server
        let stunc = STUNCommunicator::new(stun_server.to_string());
        match command.command_type {
            CommandType::Help => HelpService::run(),
            CommandType::Version => VersionService::run(),
            CommandType::CreateUser => CreateUserService::run(command),
            CommandType::Login => {
                let ls = LoginService::new(rc, fs);
                ls.run(command)
            },
            CommandType::RegisterDevice => {
                let register_device_service = RegisterDeviceService::new(rc, fs);
                register_device_service.run(command)
            },
            CommandType::Listen => {
                let ls = ListenService::new(tcpc, stunc);
                ls.run()
            }
            CommandType::Send => {
                let send_file_service = SendFileService::new(rc, fs, udtc, stunc);
                send_file_service.run(command)
            },
        }
    }
}
