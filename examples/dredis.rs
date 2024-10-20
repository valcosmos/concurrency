use std::{io::ErrorKind, net::SocketAddr};

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: Listening on {}", listener.local_addr()?);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Dredis: Connection from {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("Error processing connection from {}: {}", raddr, e);
            }
        });
    }

    // Ok(())
}

async fn process_redis_conn(mut stream: TcpStream, raddr: SocketAddr) -> anyhow::Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("Read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }
    }
    warn!("Connection from {} closed", raddr);
    Ok(())
}
