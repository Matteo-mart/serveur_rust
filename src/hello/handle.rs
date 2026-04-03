use bytes::Bytes;
use http_body_util::Full;
use hyper::{Response, StatusCode};
// use crate::response::handle_response;
use crate::hello::response::handle_response;

pub async fn handle_get() -> Response<Full<Bytes>> {
    handle_response(StatusCode::OK, "\nRéponse GET\n")
}

pub async fn handle_post() -> Response<Full<Bytes>> {
    handle_response(StatusCode::OK, "\nRéponse POST\n")
}

pub async fn handle_delete() -> Response<Full<Bytes>> {
    handle_response(StatusCode::OK, "\nRéponse DELETE\n")
}