use hyper::{Response, StatusCode};
use http_body_util::Full;
use bytes::Bytes;


pub fn handle_response(status: StatusCode, msg: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(msg))) 
        .unwrap()
}
