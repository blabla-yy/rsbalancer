use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParameterError {
    message: String,
}

#[derive(Debug)]
pub struct NotFoundError;

#[derive(Debug)]
pub struct DuplicatedKeyError;


impl ParameterError {
    pub fn new(message: &str) -> ParameterError {
        ParameterError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParameterError: {}", self.message)
    }
}

impl Error for ParameterError {
    fn description(&self) -> &str {
        &self.message
    }
}


impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not found")
    }
}

impl Error for NotFoundError {}

impl fmt::Display for DuplicatedKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Duplicated key")
    }
}

impl Error for DuplicatedKeyError {}