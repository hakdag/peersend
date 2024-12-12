use std::fs::File;
use std::io::{prelude::*, Error};
use std::path::Path;

pub trait TokenStorageAccessable {
    fn save_token(&self, token: String) -> Result<(), Error>;
}

pub struct FileStorage {}

impl TokenStorageAccessable for FileStorage {
    fn save_token(&self, token: String) -> Result<(), Error> {
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
}
