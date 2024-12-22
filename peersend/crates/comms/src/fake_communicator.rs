use core::protocol::ProtocolAccessable;

pub struct FakeCommunicator {}

impl FakeCommunicator {
    pub fn new() -> Self {
        Self { }
    }
}

impl ProtocolAccessable for FakeCommunicator {
    fn send_file(&self, _ip_address: String, _arg_filename: String) -> Result<(), std::io::Error> {
        println!("Communicating...");
        
        Ok(())
    }
    
    fn listen_file(&self, _ip_address: String) -> Result<(), std::io::Error> {
        Ok(())
    }
}