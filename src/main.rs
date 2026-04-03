use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Serveur sur http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
                .ok();
        });
    }
}

async fn hello<B>(req: Request<B>) -> Result<Response<Full<Bytes>>, Infallible> {
    match req.method() {
        &Method::GET => Ok(Response::new(Full::new(Bytes::from("Hello, World!")))),
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Full::new(Bytes::from("Méthode non autorisée")))
            .unwrap()),
    }
}