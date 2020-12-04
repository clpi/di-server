pub mod header;
pub mod cookie;
pub mod method;

pub use method::Method;
pub use header::Headers;
pub use cookie::Cookie;

use std::{
    convert::{Infallible, TryFrom},
    fmt
};

#[derive(Debug, Clone, Copy)]
pub enum HttpParseError {
    InvalidVersion,
    InvalidMethod,
    InvalidRoute,
}

#[derive(Debug, Clone, Copy)]
pub enum Version {
    V1_1, V2_0, V2_2
}

impl Default for Version {
    fn default() -> Self {
        Version::V1_1
    }
}

impl TryFrom<String> for Version {

    type Error = HttpParseError;

    fn try_from(version: String)
        -> Result<Self, HttpParseError>
    {
        let vers = match version.as_str() {
            "HTTP/1.1" => Self::V1_1,
            "HTTP/2.0" => Self::V2_0,
            "HTTP/2.2" => Self::V2_2,
            _ => { return Err(HttpParseError::InvalidVersion); }
        };
        Ok(vers)
    }
}

impl<'a> Into<&'a str> for Version {
    fn into(self) -> &'a str {
        match self {
            Self::V1_1 => "HTTP/1.1",
            Self::V2_0 => "HTTP/2.0",
            Self::V2_2 => "HTTP/2.2",
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
#[derive(Debug, Clone)]
pub enum Protocol {
    TCP, UDP,
}

impl From<&str> for Protocol {
    fn from(kind: &str) -> Self {
        match kind {
            "tcp" | "TCP" => Protocol::TCP,
            "udp" | "UDP" => Protocol::UDP,
            _ => Protocol::TCP,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HttpRun {
    Server(Protocol),
    Client(Protocol)
}

impl fmt::Display for Protocol {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Protocol::UDP => print!("UDP"),
            Protocol::TCP => print!("TCP")
        };
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
