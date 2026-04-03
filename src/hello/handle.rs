use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, StatusCode, Method};
use hyper::body::Incoming;

pub fn handle_response(status: StatusCode, msg: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::from(msg)))
        .unwrap()
}

pub async fn handle_get() -> Response<Full<Bytes>> {
    handle_response(StatusCode::OK, "\nRéponse GET\n")
}

pub async fn handle_post() -> Response<Full<Bytes>> {
    handle_response(StatusCode::OK, "\nRéponse POST\n")
}

pub async fn handle_delete() -> Response<Full<Bytes>> {
    handle_response(StatusCode::OK, "\nRéponse DELETE\n")
}


pub async fn router(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let path = req.uri().path();
    let method = req.method();

    match (method, path) {
        (&Method::GET, "/get") => Ok(handle_get().await),

        (&Method::POST, "/post") => Ok(handle_post().await),

        (&Method::DELETE, "/delete") => Ok(handle_delete().await),

        _ => {
            Ok(handle_response(StatusCode::NOT_FOUND, "Route non trouvée"))
        }
    }
}