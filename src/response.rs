use serde::{Serialize, Deserialize};
use serde_json;
use std::io;

use crate::request::{RequestBody, Request};

pub trait Service {
    type RequestBody: Into<RequestBody>;
    type ResponseBody: Into<String>;
    type Err: Into<io::Error>;
    type Future: std::future::Future;
    fn call(&mut self, request: Request<Self::RequestBody>) -> Self::Future;
}

pub struct Response<'a, K> {
    data: &'a ResponseData,
    kind: K,
}

impl<'a, K: Into<String>> Response<'a, K> {
    pub fn execute<F, R>(self, f: F) -> R
        where
            F: FnOnce() -> R,
            F::Output: Into<K>,
            R: Into<String> {
        f().into()
    }

}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {

}

#[derive(Default)]
pub struct HttpResponse {

}

impl<R> Into<HttpResponse> for Request<R>
    where
        R: Into<RequestBody> {
    fn into(self) -> HttpResponse {
        HttpResponse::default()
    }
}

// impl<'a> From<Request> for Box<dyn Response<'a>> {

// }
