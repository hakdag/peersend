use std::io::Error;

pub trait ProtocolAccessable {
    fn send_file(&self, ip_address: &String, arg_filename: String) -> Result<(), Error>;
    fn listen_file(&self, ip_address: &String) -> Result<(), Error>;
}