use core::{protocol::ProtocolAccessable, stun::STUNAccessible};
use std::io::Error;

pub struct ListenService<TProtocol, TSTUNAccessable> where TProtocol: ProtocolAccessable, TSTUNAccessable: STUNAccessible {
    protocol_access: TProtocol,
    stun_access: TSTUNAccessable
}

impl<TProtocol, TSTUNAccessable> ListenService<TProtocol, TSTUNAccessable> where TProtocol: ProtocolAccessable, TSTUNAccessable: STUNAccessible {
    pub fn new(protocol_access: TProtocol, stun_access: TSTUNAccessable) -> Self {
        Self { protocol_access, stun_access }
    }

    pub fn run(&self) -> Result<String, Error> {

        /*
        1- get public ip from stun
        2- tell server public ip
        3- start listening
        */

        let public_addr = match self.stun_access.discover_public_address() {
            Ok(addr) => addr,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Connection to STUN server failed. Could not get public address.".to_string())),
        };
        println!("Target public address: {}", public_addr);
    
        let ip = local_ip::get().unwrap().to_string();
        match self.protocol_access.listen_file(&ip) {
            Ok(_) => Ok("Done!".to_string()),
            Err(e) => Err(e),
        }
}
}
