use std::fmt;

#[derive(Debug)]
pub struct FileSystemError {
    filename: String,
    message: String,
}

impl FileSystemError {
    pub fn new(filename: String, message: String) -> Self {
        Self { filename, message }
    }
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error opening file '{}': {}", self.filename, self.message)
    }
}

impl From<std::io::Error> for FileSystemError {
    fn from(err: std::io::Error) -> Self {
        FileSystemError {
            filename: String::from("unknown"),
            message: err.to_string(),
        }
    }
}
