use bytes::Bytes;
use http_body_util::Full;
use hyper::{Response, StatusCode};

use crate::hello::hello;

pub async fn handle_get() -> Response<Full<Bytes>> {
    hello::handle_response(StatusCode::OK, "\nRéponse GET\n")
}

pub async fn handle_post() -> Response<Full<Bytes>> {
    hello::handle_response(StatusCode::OK, "\nRéponse POST\n")
}

pub async fn handle_delete() -> Response<Full<Bytes>> {
    hello::handle_response(StatusCode::OK, "\nRéponse DELETE\n")
}