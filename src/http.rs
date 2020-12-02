pub mod header;
pub mod cookie;
pub mod method;

pub use header::Headers;
use std::convert::{Infallible, TryFrom};

#[derive(Debug, Clone,)]
pub enum Method {
    GET,
    POST,
    PUT,
    OPTIONS,
    HEAD,
    DELETE,
}

#[derive(Debug, Clone)]
pub enum  HttpVersion {
    V1_1, V2_0, V2_2
}

impl Default for HttpVersion {
    fn default() -> Self {
        HttpVersion::V1_1
    }
}

impl Default for Method {
    fn default() -> Self {
        Self::GET
    }
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

pub enum HttpRequest {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    NONE,
}
