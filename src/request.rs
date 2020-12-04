use std::{
    fs, fmt
};
use crate::route::Route;
use dhttp::{
    Version, Method,
    header::{Header, Headers}
};

#[derive(Debug, Clone)]
pub struct Request<R> {
    pub method: Method,
    pub headers: Vec<Header>,
    pub route: Route,
    pub http_version: Version,
    pub body: R,
}

pub struct RequestBody {}

impl<R: Into<RequestBody> + Default> Default for Request<R> {
    fn default() -> Self {
        Self {
            body: R::default(), ..Default::default()
        }
    }
}

impl<R: Into<RequestBody> + Default> Request<R> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<R: Into<RequestBody> + Default> From<String> for Request<R> {
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

impl fmt::Display for Request<&str> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.http_version.into())?;
        f.write_str(self.method.clone().into())?;
        f.write_str(self.body)?;
        Ok(())
    }
}

pub trait Body {

}
