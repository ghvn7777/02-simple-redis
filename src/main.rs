use anyhow::Result;
use simple_redis::{network, Backend};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Simple-Redis-Server is listening on {}", addr);

    let backend = Backend::new();
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);

        let clone_backend = backend.clone();
        tokio::spawn(async move {
            match network::stream_handler(stream, clone_backend).await {
                Ok(_) => info!("Connection from {} closed", raddr),
                Err(e) => warn!("Handle error for {}: {:?}", addr, e),
            }
        });
    }
}
