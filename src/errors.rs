use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NotFoundError;

#[derive(Debug)]
pub struct DuplicatedKeyError;


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