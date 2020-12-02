use std::fs;
use dhttp::{HttpVersion, Method};

#[derive(Debug, Clone, Default)]
pub struct Request<'a> {
    method: Method,
    uri: &'a str,
    http_version: HttpVersion,
}

impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> From<String> for Request<'a> {
    fn from(req: String) -> Self {
        match req.lines().next() {
            Some(req_line) => {
                match req_line.split(" ").next() {
                    Some("GET") => Self::default(),
                    Some("POST") => Self::default(),
                    _ => Self::default()
                }
            },
            None => Self::default(),
        }
    }
}
