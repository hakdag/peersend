use std::io::Error;
use core::{command::Command, device::Device, storage::StorageAccess, user::User};
use crate::{file::TokenStorageAccessable, get_arg, jwt::TokenHandler};
extern crate local_ip;

pub struct RegisterDeviceService<TRedis, TFile> where TRedis: StorageAccess, TFile: TokenStorageAccessable {
    storage_access: TRedis,
    token_storage_access: TFile
}

impl<TRedis, TFile> RegisterDeviceService<TRedis, TFile> where TRedis: StorageAccess, TFile: TokenStorageAccessable {
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

        // detect current device's ip address
        let ip = local_ip::get().unwrap();

        // create device obj
        let arguments = match &command.arguments {
            Some(args) => args,
            None => &Vec::new(),
        };
        let device = Device::new(get_arg(arguments, 0), Some(ip.to_string()));
        
        // add device obj to user's devices list
        let mut user: User = match self.storage_access.get(user_id) {
            Ok(u) => u,
            Err(e) => return Result::Err(e),
        };
        user.add_device(device);

        // save user
        match self.storage_access.set(user.user_name().to_string(), user) {
            Ok(_) => Result::Ok("Device is registered.".to_string()),
            Err(e) => Result::Err(e),
        }
    }
}