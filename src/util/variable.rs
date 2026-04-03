use tokio::net::TcpListener;
use std::net::SocketAddr;

pub async fn variable() -> Result<(TcpListener, SocketAddr), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    Ok((listener, addr))
}