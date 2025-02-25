use core::command::{Command, CommandType};
use std::io::Error;
use comms::api_communicator::APICommunicator;
use comms::stun_communicator::STUNCommunicator;
use comms::tcp_communicator::TCPCommunicator;
use comms::udt_communicator::UDTCommunicator;
use services::file::FileStorage;
use services::help::HelpService;
use services::listen::ListenService;
use services::user::UserService;
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
        // let fs = FileStorage {};
        let rc = match RedisCommunication::new() {
            Ok(rc) => rc,
            Err(e) => return Err(e),
        };
        let tcpc = TCPCommunicator::new();
        let udtc = UDTCommunicator::new();
        let stun_server = "stun.l.google.com:19302"; // Example STUN server
        let stunc = STUNCommunicator::new(stun_server.to_string());
        let api = APICommunicator::<FileStorage>::new(FileStorage::new());
        let user = UserService::new(rc, FileStorage::new());
        match command.command_type {
            CommandType::Help => HelpService::run(),
            CommandType::Version => VersionService::run(),
            CommandType::CreateUser => {
                let rc1 = match RedisCommunication::new() {
                    Ok(rc) => rc,
                    Err(e) => return Err(e),
                };
                return CreateUserService::new(rc1).run(command);
            },
            CommandType::Login => {
                let rc2 = match RedisCommunication::new() {
                    Ok(rc) => rc,
                    Err(e) => return Err(e),
                };
                let ls = LoginService::new(rc2, FileStorage::new());
                ls.run(command)
            },
            CommandType::RegisterDevice => {
                let rc3 = match RedisCommunication::new() {
                    Ok(rc) => rc,
                    Err(e) => return Err(e),
                };
                let register_device_service = RegisterDeviceService::new(rc3, FileStorage::new());
                register_device_service.run(command)
            },
            CommandType::Listen => {
                let ls = ListenService::new(tcpc, stunc, api);
                ls.run()
            }
            CommandType::Send => {
                let rc4 = match RedisCommunication::new() {
                    Ok(rc) => rc,
                    Err(e) => return Err(e),
                };
                let send_file_service = SendFileService::new(rc4, udtc, stunc, user, api);
                send_file_service.run(command)
            },
        }
    }
}
