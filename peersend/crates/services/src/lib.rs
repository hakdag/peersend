use std::io::{Error, ErrorKind};
use mac_address::get_mac_address;

use core::command::CommandArgument;

pub mod help;
pub mod version;
pub mod create_user;
pub mod login;
pub mod register_device;
pub mod send_file;
pub mod jwt;
pub mod file;
pub mod listen;
pub mod user;

pub fn get_arg(arguments: &Vec<CommandArgument>, index: usize) -> String {
    match arguments.get(index) {
        Some(arg) => arg.name.clone(),
        None => String::new(),
    }
}

pub fn get_mac() -> Result<String, Error> {
    match get_mac_address() {
        Ok(Some(ma)) => {
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
            Ok(ma.to_string())
        }
        Ok(None) => return Err(Error::new(ErrorKind::Other, "No MAC address found.")),
        Err(e) => return Err(Error::new(ErrorKind::Other, e)),
    }
}