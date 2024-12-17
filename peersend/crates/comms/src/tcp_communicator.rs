use core::protocol::ProtocolAccessable;
use std::{fs::File, io::{BufReader, Read, Write}, net::{TcpListener, TcpStream}, thread};

pub struct TCPCommunicator {}

impl TCPCommunicator {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_client(mut stream: TcpStream) {
        let mut buffer = Vec::new();
        match stream.read_to_end(&mut buffer) {
            Ok(_) => {
                let file_path = "received_file.json";
                let mut file = File::create(file_path).expect("Failed to create file");
                file.write_all(&buffer).expect("Failed to write to file");
                println!("File received and saved as {}", file_path);
            }
            Err(e) => {
                eprintln!("Failed to read from server: {}", e);
            }
        }
    }
}

impl ProtocolAccessable for TCPCommunicator {
    fn send_file(&self, ip_address: String, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        println!("Communicating...");
        match TcpStream::connect(format!("{ip_address}:1234")) {
            Ok(mut stream) => {
                println!("Connected to device.");
    
                match stream.write_all(&buffer) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e)
        }
    }
    
    fn listen(&self, ip_address: String) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(format!("{ip_address}:1234")).expect("Failed to bind address");
        println!("Listening file transfer...");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Connection detected!");
                    let handler = thread::spawn(|| Self::handle_client(stream));
                    handler.join().unwrap();
                    return Ok(())
                }
                Err(e) => return Err(e),
            }
        };
        Ok(())
    }
}