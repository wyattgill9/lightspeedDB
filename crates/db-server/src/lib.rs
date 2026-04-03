use std::net::SocketAddr;

use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::TcpListener;

#[tokio::main]
async fn run_server_main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.bind(&addr.into())?;

    let listener = TcpListener::from_std(socket.into())?;

    loop {
        let (_socket, _addr) = listener.accept().await?;
        break;
    }

    Ok(())
}
