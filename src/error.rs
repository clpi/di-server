use std::{
    error::Error, fmt,
    convert::TryFrom,
};

#[derive(Debug)]
pub enum SrvError {
    NotFound,
    InternalError,
}

impl fmt::Display for SrvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SrvError::NotFound => f.write_str("Not found"),
            SrvError::InternalError => f.write_str("Internal Server Error"),
        }
    }
}

impl Error for SrvError {
    fn description(&self) -> &str {
        match *self {
            SrvError::NotFound => "Not found",
            SrvError::InternalError => "Internal server error",
        }
    }
}
