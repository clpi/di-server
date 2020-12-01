use crate::request::Request;
pub trait Response<'a> {
    fn new() -> Self;
}

#[derive(Default)]
pub struct HttpResponse {

}

impl<'a> Into<HttpResponse> for Request<'a>{
    fn into(self) -> HttpResponse {
        HttpResponse::default()
    }
}



