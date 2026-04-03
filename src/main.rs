use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;

use crate::hello::handle;
mod util;
mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let (listener, addr) = util::variable::variable().await?;
    
    println!("Serveur actif sur http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        
        tokio::task::spawn(async move {
            let io = TokioIo::new(stream);
            
            let conn = http1::Builder::new()
                .serve_connection(io, service_fn(|req| {
                    handle::router(req)
                }));

            if let Err(e) = conn.await {
                eprintln!("Erreur de connexion: {e}");
            }
        });
    }
}