use std::{io::Error, path::Path};
use core::{api::ApiAccess, command::Command, protocol::ProtocolAccessable, storage::StorageAccess, stun::STUNAccessible, user::{User, UsersAccessable}};
use crate::get_arg;

pub struct SendFileService<TRedis, TProtocol, TSTUNAccessable, TUsersAccessable, TApiAccess>
    where TRedis: StorageAccess,
        TProtocol: ProtocolAccessable,
        TSTUNAccessable: STUNAccessible,
        TUsersAccessable: UsersAccessable,
        TApiAccess: ApiAccess {
    storage_access: TRedis,
    protocol_access: TProtocol,
    stun_access: TSTUNAccessable,
    users_access: TUsersAccessable,
    api_access: TApiAccess
}

impl<TRedis, TProtocol, TSTUNAccessable, TUsersAccessable, TApiAccess> SendFileService<TRedis, TProtocol, TSTUNAccessable, TUsersAccessable, TApiAccess>
    where TRedis: StorageAccess,
        TProtocol: ProtocolAccessable,
        TSTUNAccessable: STUNAccessible,
        TUsersAccessable: UsersAccessable,
        TApiAccess: ApiAccess {
    pub fn new(storage_access: TRedis,
        protocol_access: TProtocol,
        stun_access: TSTUNAccessable,
        users_access: TUsersAccessable,
        api_access: TApiAccess) -> Self {
        Self { storage_access, protocol_access, stun_access, users_access, api_access }
    }

    pub fn run(&self, command: &Command) -> Result<String, Error> {

        /*
        1- get public ip from stun
        2- tell server public ip
        3- get target device's public ip from the server
        4- send the file
        */

        let public_addr = match self.stun_access.discover_public_address() {
            Ok(addr) => addr,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Connection to STUN server failed. Could not get public address.".to_string())),
        };
        println!("Source public address: {}", public_addr);

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

        let user = match self.users_access.get_user() {
            Ok(u) => u,
            Err(e) => return Err(e),
        };
        if !user.has_device(&arg_source_device) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "User did not register the source device.".to_string()));
        }

        if !user.has_device(&arg_target_device) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "User did not register the target device.".to_string()));
        }

        self.send_file(user, arg_filename, arg_target_device)
    }

    fn send_file(&self, user: User, arg_filename: String, arg_target_device: String) -> Result<String, Error> {
        let target_device = match user.get_device_by_name(&arg_target_device) {
            Some(d) => d,
            None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Device not found.".to_string())),
        };
        
        let target_device_ip_address: String = match self.api_access.get_target_ipaddress(&arg_target_device) {
            Ok(addr) => addr,
            Err(e) => return Err(e)
        };
        
        match self.protocol_access.send_file(&target_device_ip_address, arg_filename) {
            Ok(_) => Ok("File sent!".to_string()),
            Err(e) => Err(e),
        }
    }
}