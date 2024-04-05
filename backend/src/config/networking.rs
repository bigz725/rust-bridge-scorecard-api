use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn listener(addr: SocketAddr) -> TcpListener {
    TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port")
}

pub fn bind_addr() -> SocketAddr {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port_int = port.parse::<u16>().unwrap();
    SocketAddr::from(([0, 0, 0, 0], port_int))
}
