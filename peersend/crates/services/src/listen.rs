use core::protocol::ProtocolAccessable;
use std::io::Error;

pub struct ListenService<TProtocol> where TProtocol: ProtocolAccessable {
    protocol_access: TProtocol
}

impl<TProtocol> ListenService<TProtocol> where TProtocol: ProtocolAccessable {
    pub fn new(protocol_access: TProtocol) -> Self {
        Self { protocol_access }
    }

    pub fn run(&self) -> Result<String, Error> {

        /*
        1- get public ip from stun
        2- tell server public ip
        3- start listening
        */


        let ip = local_ip::get().unwrap().to_string();
        match self.protocol_access.listen_file(&ip) {
            Ok(_) => Ok("Done!".to_string()),
            Err(e) => Err(e),
        }
}
}
