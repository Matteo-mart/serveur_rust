use std::convert::Infallible;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Request, Response, StatusCode};

use crate::hello::handle;


pub fn handle_response(status: StatusCode, msg: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(msg)))
        .unwrap()
}

pub async fn hello<B>(req: Request<B>) -> Result<Response<Full<Bytes>>, Infallible> {
    let response = match *req.method() {
        Method::GET    => handle::handle_get().await,
        Method::POST   => handle::handle_post().await,
        Method::DELETE => handle::handle_delete().await,
        _              => handle_response(StatusCode::METHOD_NOT_ALLOWED, "\nNon autorisé\n"),
    };

    Ok(response)
}