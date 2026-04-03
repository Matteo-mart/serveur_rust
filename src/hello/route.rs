use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, StatusCode, Method};
use hyper::body::Incoming;

use crate::route;
use crate::hello::handle::handle_get;
use crate::hello::handle::handle_post;
use crate::hello::handle::handle_delete;
use crate::hello::response;

pub async fn router(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let path = req.uri().path();
    let method = req.method();

    match (method, path) {
        (&Method::GET, "/get") => Ok(route::handle_get().await),

        (&Method::POST, "/post") => Ok(route::handle_post().await),

        (&Method::DELETE, "/delete") => Ok(route::handle_delete().await),

        _ => {
            Ok(response::handle_response(StatusCode::NOT_FOUND, "\nRoute non trouvée\n"))
        }
    }
}