use serde::{Serialize, Deserialize};
use serde_json;

use crate::request::Request;

pub trait Response<'a> {
}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {

}

#[derive(Default)]
pub struct HttpResponse {

}

impl<'a> Into<HttpResponse> for Request<'a>{
    fn into(self) -> HttpResponse {
        HttpResponse::default()
    }
}

impl<'a> ToString for Box<dyn Response<'a>> {
    fn to_string(&self) -> String {
        serde_json::to_string_pretty("{}").unwrap()
    }

}

// impl<'a> From<Request> for Box<dyn Response<'a>> {

// }
