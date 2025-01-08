use core::protocol::ProtocolAccessable;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Write};
use std::time::Duration;
use stunclient::StunClient;

pub struct UDTCommunicator {}

impl UDTCommunicator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn discover_public_address(&self, stun_server: &str) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        // Parse the STUN server address
        let stun_addr = stun_server
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

impl ProtocolAccessable for UDTCommunicator {
    fn send_file(&self, ip_address: &String, arg_filename: String) -> Result<(), Error> {
        let stun_server = "stun.l.google.com:19302"; // Example STUN server
        let public_addr = match self.discover_public_address(stun_server) {
            Ok(addr) => addr,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Connection to STUN server failed. Could not get public address.".to_string())),
        };
        println!("Source public address: {}", public_addr);
    
        // Target public address (replace with actual target's public IP:PORT)
        // let target_addr = "127.0.0.1:8080";
        println!("Attempting to connect to target: {}", ip_address);
    
        let local_addr = "0.0.0.0:0";
        let socket = match UdpSocket::bind(local_addr) {
            Ok(s) => s,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, format!("Couldn't bind to address: {local_addr}."))),
        };

        // Handshake: Ping the target and wait for a response
        let handshake_message = b"PING";
        match socket.send_to(handshake_message, ip_address) {
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Handshake with target failed.".to_string())),
        };
    
        match socket.set_read_timeout(Some(Duration::from_secs(5))) {
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Set a timeout for the response with target failed.".to_string())),
        };

        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf) {
            Ok((len, _)) => {
                let response = String::from_utf8_lossy(&buf[..len]).trim().to_string();
                if response == "PONG" {
                    println!("Handshake successful. Target is ready.");
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, format!("Unexpected response from target: {}", response)));
                }
            }
            Err(_) => {
                return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Target did not respond. Please ensure the target is running.".to_string()));
            }
        }
    
        println!("Sending data to: {}", ip_address);
        
        match socket.send_to(arg_filename.as_bytes(), ip_address) {
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Could not send file name to the target.".to_string())),
        };
        println!("Sent filename: {}", arg_filename);
    
        let file = match File::open(arg_filename) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        let mut reader = BufReader::new(file);
        let mut buf = vec![0u8; 1024];
    
        loop {
            let len = match reader.read(&mut buf) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };
            if len == 0 {
                break; // End of file
            }

            match socket.send_to(&buf[..len], ip_address) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
    
        // Send EOF marker to signal end of transmission
        match socket.send_to(b"EOF", ip_address) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        println!("File sent successfully!");

        Ok(())
    }

    fn listen_file(&self, ip_address: &String) -> Result<(), Error> {
        let stun_server = "stun.l.google.com:19302";
        let public_addr = match self.discover_public_address(stun_server) {
            Ok(addr) => addr,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Connection to STUN server failed. Could not get public address.".to_string())),
        };
    
        // Bind to a local address
        let local_addr = "127.0.0.1:8080";
        let socket = match UdpSocket::bind(local_addr) {
            Ok(s) => s,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, format!("Couldn't bind to address: {local_addr}."))),
        };
        println!("Target listening on local address: {}", socket.local_addr()?);
    
        let mut buf = vec![0u8; 1024];
        
        // Handshake: Respond to PING with PONG
        let (len, src_addr) = match socket.recv_from(&mut buf) {
            Ok(r) => r,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Handshake with source failed.".to_string())),
        };
        let message = String::from_utf8_lossy(&buf[..len]).trim().to_string();
        if message == "PING" {
            println!("Received handshake request from: {}", src_addr);
            match socket.send_to(b"PONG", src_addr) {
                Ok(_) => (),
                Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Handshake with source failed.".to_string())),
            };
            println!("Sent handshake response: PONG");
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, format!("Unexpected handshake message: {}", message)));
        }
    
        // Print to check if target is ready
        let (len, _) = match socket.recv_from(&mut buf) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        println!("Received filename: {}", String::from_utf8_lossy(&buf[..len]));
        
        let filename = String::from_utf8_lossy(&buf[..len]).trim().to_string();
        let file = match File::create(filename) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        let mut writer = BufWriter::new(file);
    
        loop {
            let (len, _) = socket.recv_from(&mut buf)?;
            if len == 0 || buf.starts_with(b"EOF") {
                println!("End of file transmission detected.");
                break;
            }
            writer.write_all(&buf[..len])?;
        }
    
        writer.flush()?;
        println!("File received successfully!");
        Ok(())
    }
}
