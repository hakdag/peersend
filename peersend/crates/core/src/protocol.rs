use std::io::Error;

pub trait ProtocolAccessable {
    fn send_file(&self, ip_address: String, buffer: &Vec<u8>) -> Result<(), Error>;
    fn listen(&self, ip_address: String) -> Result<(), Error>;
}