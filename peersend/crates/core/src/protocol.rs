use std::io::Error;

pub trait ProtocolAccessable {
    fn send_file(&self, buffer: &Vec<u8>) -> Result<(), Error>;
}