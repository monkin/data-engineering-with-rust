use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::new(&format!("IO error: {}", error))
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::new(&format!("Reqwest error: {}", error))
    }
}
