use std::fmt;

#[derive(Debug)]
pub struct UnauthorizedError {
    message: String,
}

impl UnauthorizedError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for UnauthorizedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Security error: {}", self.message)
    }
}

impl From<std::io::Error> for UnauthorizedError {
    fn from(err: std::io::Error) -> Self {
        UnauthorizedError {
            message: err.to_string(),
        }
    }
}
