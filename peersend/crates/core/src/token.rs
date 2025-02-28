use std::io::Error;

pub trait TokenStorageAccessable {
    fn save(&self, token: String) -> Result<(), Error>;
    fn read(&self) -> Result<String, Error>;
}
