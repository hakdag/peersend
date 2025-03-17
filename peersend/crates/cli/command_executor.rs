use core::command::{Command, CommandType};
use std::io::Error;
use comms::api_communicator::APICommunicator;
use comms::stun_communicator::STUNCommunicator;
use comms::protocols::tcp_communicator::TCPCommunicator;
use comms::protocols::udt_communicator::UDTCommunicator;
use services::file::FileStorage;
use services::help::HelpService;
use services::listen::ListenService;
use services::register_device::RegisterDeviceService;
use services::user::UserService;
use services::version::VersionService;
use services::create_user::CreateUserService;
use services::login::LoginService;
use services::send_file::SendFileService;

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn execute(command: &Command) -> Result<String, Error> {
        println!("Executing command: {}", command.name);
        let tcpc = TCPCommunicator::new();
        let udtc = UDTCommunicator::new();
        let stun_server = "stun.l.google.com:19302"; // Example STUN server
        let stunc = STUNCommunicator::new(stun_server.to_string());
        let user = UserService::new(FileStorage::new(), create_api());
        match command.command_type {
            CommandType::Help => HelpService::run(),
            CommandType::Version => VersionService::run(),
            CommandType::CreateUser => CreateUserService::new(create_api()).run(command),
            CommandType::Login => LoginService::new(create_api(), FileStorage::new()).run(command),
            CommandType::RegisterDevice => RegisterDeviceService::new(create_api(), FileStorage::new()).run(command),
            CommandType::Listen => ListenService::new(tcpc, stunc, create_api()).run(),
            CommandType::Send => SendFileService::new(udtc, stunc, user, create_api()).run(command),
        }
    }
}

fn create_api() -> APICommunicator<FileStorage> {
    APICommunicator::<FileStorage>::new(FileStorage::new(), "http://127.0.0.1:8080".to_string())
}
