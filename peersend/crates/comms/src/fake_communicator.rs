use core::protocol::ProtocolAccessable;

pub struct FakeCommunicator {}

impl FakeCommunicator {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProtocolAccessable for FakeCommunicator {
    fn send_file(&self, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        println!("Communicating...");
        
        Ok(())
    }
}