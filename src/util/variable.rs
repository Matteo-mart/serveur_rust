use tokio::net::TcpListener;
use std::net::SocketAddr;

// On rend la fonction async et on retourne le listener et l'adresse
pub async fn variable() -> Result<(TcpListener, SocketAddr), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    Ok((listener, addr))
}