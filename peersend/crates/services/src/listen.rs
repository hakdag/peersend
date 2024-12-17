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
        let ip = local_ip::get().unwrap();
        match self.protocol_access.listen(ip.to_string()) {
            Ok(_) => Ok("Done!".to_string()),
            Err(e) => Err(e),
        }
    }
}
