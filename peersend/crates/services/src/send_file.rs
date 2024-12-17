use std::{fs::File, io::{BufReader, Error, Read}, path::Path};
use core::{command::Command, protocol::ProtocolAccessable, storage::StorageAccess, user::User};
use crate::{file::TokenStorageAccessable, get_arg, jwt::TokenHandler};

pub struct SendFileService<TRedis, TFile, TProtocol> where TRedis: StorageAccess, TFile: TokenStorageAccessable, TProtocol: ProtocolAccessable {
    storage_access: TRedis,
    token_storage_access: TFile,
    protocol_access: TProtocol
}

impl<TRedis, TFile, TProtocol> SendFileService<TRedis, TFile, TProtocol> where TRedis: StorageAccess, TFile: TokenStorageAccessable, TProtocol: ProtocolAccessable {
    pub fn new(storage_access: TRedis, token_storage_access: TFile, protocol_access: TProtocol) -> Self {
        Self { storage_access, token_storage_access, protocol_access }
    }

    pub fn run(&self, command: &Command) -> Result<String, Error> {
        // read token
        let token = match self.token_storage_access.read() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        // validate token
        // get user Id from token's sub claim
        let token_handler = TokenHandler::new();
        let user_id = match token_handler.validate(token) {
            Ok(id) => id,
            Err(e) => return Err(e),
        };
        let user: User = match self.storage_access.get(user_id) {
            Ok(u) => u,
            Err(e) => return Result::Err(e),
        };

        let arguments = match &command.arguments {
            Some(args) => args,
            None => &Vec::new(),
        };
        let arg_filename = get_arg(arguments, 0);
        let arg_source_device = get_arg(arguments, 1);
        let arg_target_device = get_arg(arguments, 2);

        let path = Path::new(&arg_filename);
        if !path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "File not found.".to_string()));
        }

        if !user.has_device(&arg_source_device) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "User did not register the source device.".to_string()));
        }

        if !user.has_device(&arg_target_device) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "User did not register the target device.".to_string()));
        }

        let buffer = match Self::get_file_stream(&arg_filename) {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        let target_device = match user.get_device_by_name(&arg_target_device) {
            Some(d) => d,
            None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Device not found.".to_string())),
        };
        
        match self.protocol_access.send_file(target_device.ip_address.as_ref().unwrap().to_string(), &buffer) {
            Ok(_) => Ok("File sent!".to_string()),
            Err(e) => Err(e),
        }
    }

    fn get_file_stream(filename: &String) -> Result<Vec<u8>, Error> {
        let file = match File::open(&filename) {
            Ok(f) => f,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to open file.".to_string())),
        };
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
    
        let _ = match reader.read_to_end(&mut buffer) {
            Ok(n) => n,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to read file.".to_string())),
        };

        Ok(buffer)
    }
}