use core::stun::STUNAccessible;
use std::net::{ToSocketAddrs, UdpSocket};
use stunclient::StunClient;

pub struct STUNCommunicator {
    stun_server: String
}

impl STUNCommunicator {
    pub fn new(stun_server: String) -> Self {
        Self { stun_server }
    }
}

impl STUNAccessible for STUNCommunicator {
    fn discover_public_address(&self) -> Result<std::net::SocketAddr, Box<dyn std::error::Error>> {
        // Parse the STUN server address
        let stun_addr = self.stun_server
            .to_socket_addrs()
            .unwrap()
            .filter(|x|x.is_ipv4())
            .next()
            .unwrap();
        
        // Bind a local UDP socket to communicate with the STUN server
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        let local_addr = socket.local_addr()?;
        println!("Local socket bound to: {}", local_addr);
    
        // Use the STUN client to query the public address
        let client = StunClient::new(stun_addr);
        let public_addr = client.query_external_address(&socket)?;
        println!("Public address discovered: {}", public_addr);
    
        Ok(public_addr)
    }
}