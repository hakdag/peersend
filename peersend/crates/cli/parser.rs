use std::env;
use std::io::Error;
use crate::command::Command;
use crate::parsers::get_arg;
use crate::parsers::help_parser::HelpParser;
use crate::parsers::listen_parser::ListenParser;
use crate::parsers::version_parser::VersionParser;
use crate::parsers::create_user_parser::CreateUserParser;
use crate::parsers::login_parser::LoginParser;
use crate::parsers::register_device_parser::RegisterDeviceParser;
use crate::parsers::send_file_parser::SendFileParser;

#[derive(Debug)]
pub struct Parser {}

impl Parser {
    pub fn parse() -> Result<Command, Error> {
        let args: Vec<String> = env::args().collect();
        let count = args.len();
        if count < 2 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not enough arguments"));
        }

        // currently supporting single command
        let arg = get_arg(&args, 1);
        match arg {
            "help" => HelpParser::parse(),
            "version" => VersionParser::parse(),
            "create-user" => CreateUserParser::parse(&args),
            "login" => LoginParser::parse(&args),
            "register" => RegisterDeviceParser::parse(&args),
            "listen" => ListenParser::parse(),
            "send" => SendFileParser::parse(&args),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown command"))
        }
    }


}

