use std::{io::Error, path::Path};
use core::{command::Command, storage::StorageAccess, user::User};
use crate::{file::TokenStorageAccessable, get_arg, jwt::TokenHandler};

pub struct SendFileService<TRedis, TFile> where TRedis: StorageAccess, TFile: TokenStorageAccessable {
    storage_access: TRedis,
    token_storage_access: TFile
}

impl<TRedis, TFile> SendFileService<TRedis, TFile> where TRedis: StorageAccess, TFile: TokenStorageAccessable {
    pub fn new(storage_access: TRedis, token_storage_access: TFile) -> Self {
        Self { storage_access, token_storage_access }
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

        if !user.has_device(arg_source_device) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "User did not register the source device.".to_string()));
        }

        if !user.has_device(arg_target_device) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "User did not register the target device.".to_string()));
        }

        Ok("File sent!".to_string())
    }
}