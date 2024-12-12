use std::fs::File;
use std::io::{prelude::*, Error};
use std::path::Path;

pub trait TokenStorageAccessable {
    fn save(&self, token: String) -> Result<(), Error>;
    fn read(&self) -> Result<String, Error>;
}

pub struct FileStorage {}

impl TokenStorageAccessable for FileStorage {
    fn save(&self, token: String) -> Result<(), Error> {
        let path = Path::new("peersend.token");
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Token file could not be created.").to_string())),
            Ok(f) => f,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(token.as_bytes()) {
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Can't write to token file.").to_string())),
            Ok(_) => Ok(()),
        }
    }

    fn read(&self) -> Result<String, Error> {
        let path = Path::new("peersend.token");

        let mut file = match File::open(&path) {
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Token file could not be opened.").to_string())),
            Ok(f) => f,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Can't read token from file.").to_string())),
            Ok(_) => Ok(s.to_string()),
        }
    }
}
