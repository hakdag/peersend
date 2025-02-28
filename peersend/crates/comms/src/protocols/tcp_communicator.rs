use core::protocol::ProtocolAccessable;
use std::{fs::File, io::{BufReader, Error, Read, Write}, net::{TcpListener, TcpStream}, thread};

pub struct TCPCommunicator {}

impl TCPCommunicator {
    pub fn new() -> Self {
        Self {}
    }

    fn get_file_stream(filename: &String) -> Result<Vec<u8>, Error> {
        let file = match File::open(&filename) {
            Ok(f) => f,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to open file.".to_string())),
        };
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
    
        let _ = match reader.read_to_end(&mut buffer) {
            Ok(n) => n,
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to read file.".to_string())),
        };

        Ok(buffer)
    }

    fn handle_client(mut stream: TcpStream) {
        // Receive file name
        let mut name_length_bytes = [0; 4];
        stream.read_exact(&mut name_length_bytes).expect("Failed to read name length");
        let name_length = u32::from_le_bytes(name_length_bytes) as usize;

        let mut file_name_bytes = vec![0; name_length];
        stream.read_exact(&mut file_name_bytes).expect("Failed to read file name");
        let file_name = String::from_utf8(file_name_bytes).expect("Failed to decode file name");
        println!("Saving file: {} ...", file_name);

        // Send ACK for file name
        stream.write_all(b"ACK").expect("Failed to send ACK");

        // Receive file content length
        let mut content_length_bytes = [0; 4];
        stream.read_exact(&mut content_length_bytes).expect("Failed to read file content length");
        let content_length = u32::from_le_bytes(content_length_bytes) as usize;

        // Receive file content
        let mut buffer = vec![0; content_length];
        stream.read_exact(&mut buffer).expect("Failed to read file content");

        // Save file
        let mut file = File::create(&file_name).expect("Failed to create file");
        file.write_all(&buffer).expect("Failed to write to file");
        println!("File '{}' received and saved", file_name);

        // Send final ACK
        stream.write_all(b"ACK").expect("Failed to send final ACK");
        println!("Transfer completed successfully");
    }
}

impl ProtocolAccessable for TCPCommunicator {
    fn send_file(&self, ip_address: &String, arg_filename: String) -> Result<(), std::io::Error> {
        print!("Connecting to device...");

        match TcpStream::connect(format!("{ip_address}:1234")) {
            Ok(mut stream) => {
                println!("Connected!");
                print!("Sending file meta...");

                // Send file name
                let file_name = arg_filename.as_bytes();
                let name_length = file_name.len() as u32;
                stream.write_all(&name_length.to_le_bytes()).expect("Failed to send name length");
                stream.write_all(file_name).expect("Failed to send file name");
                println!("Sent!");

                // Wait for ACK for file name
                let mut ack_buffer = [0; 3];
                stream.read_exact(&mut ack_buffer).expect("Failed to read ACK");
                if ack_buffer == *b"ACK" {
                    println!("ACK received from target. Sending file content...");

                    // Send file content
                    let file_buffer = match Self::get_file_stream(&arg_filename) {
                        Ok(b) => b,
                        Err(e) => return Err(e),
                    };
                    let content_length = file_buffer.len() as u32;

                    stream.write_all(&content_length.to_le_bytes()).expect("Failed to send file content length");
                    stream.write_all(&file_buffer).expect("Failed to send file content");

                    // Wait for final ACK
                    let mut final_ack_buffer = [0; 3];
                    stream.read_exact(&mut final_ack_buffer).expect("Failed to read final ACK");
                    if final_ack_buffer == *b"ACK" {
                        println!("Sent!");
                        Ok(())
                    } else {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Final ACK not received!".to_string()));
                    }
                } else {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "ACK not received for file name!".to_string()));
                }
            }
            Err(e) => Err(e)
        }
    }
    
    fn listen_file(&self, ip_address: &String) -> Result<(), std::io::Error> {
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