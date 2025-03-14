use core::{api::ApiAccess, protocol::ProtocolAccessable, stun::STUNAccessible};
use std::io::Error;

pub struct ListenService<TProtocol, TSTUNAccessable, TApiAccess>
    where TProtocol: ProtocolAccessable,
        TSTUNAccessable: STUNAccessible,
        TApiAccess: ApiAccess {
    protocol_access: TProtocol,
    stun_access: TSTUNAccessable,
    api_access: TApiAccess
}

impl<TProtocol, TSTUNAccessable, TApiAccess> ListenService<TProtocol, TSTUNAccessable, TApiAccess>
    where TProtocol: ProtocolAccessable,
        TSTUNAccessable: STUNAccessible,
        TApiAccess: ApiAccess {
    pub fn new(protocol_access: TProtocol, stun_access: TSTUNAccessable, api_access: TApiAccess) -> Self {
        Self { protocol_access, stun_access, api_access }
    }

    pub fn run(&self) -> Result<String, Error> {

        /*
        1- get public ip from stun
        2- tell server public ip, mac, user id (or email), and device name
        3- start listening
        */

        let public_addr = match self.stun_access.discover_public_address() {
            Ok(addr) => addr,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, format!("Connection to STUN server failed. Could not get public address. {}", e.to_string()))),
        };
        println!("Target public address: {}", public_addr);

        match self.api_access.set_target_ipaddress(&public_addr.ip().to_string()) {
            Ok(()) => (),
            Err(e) => return Err(e)
        };

        let ip = local_ip::get().unwrap().to_string();
        match self.protocol_access.listen_file(&ip) {
            Ok(_) => Ok("Done!".to_string()),
            Err(e) => Err(e),
        }
    }
}
