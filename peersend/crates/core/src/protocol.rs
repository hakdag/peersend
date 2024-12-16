use std::io::Error;

pub trait ProtocolAccessable {
    fn send_file(&self) -> Result<(), Error>;
}