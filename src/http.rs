use std::convert::{Infallible, TryFrom};

pub enum Method {
    GET,
    POST,
    PUT,
    OPTIONS,
    HEAD,
    DELETE,
}

impl Method {
}

impl TryFrom<&str> for Method {
    type Error = String;
    fn try_from(word: &str) -> Result<Self, Self::Error> {
        match word {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            _ => Err(format!("Invalid method: {}", word)),
        }
    }
}

impl Into<&str> for Method {
    fn into(self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::HEAD => "HEAD",
            _ => "NONE",
        }
    }
}
