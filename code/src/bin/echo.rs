use std::io::stdin;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task::spawn_blocking;
use tokio::time::sleep;

const ADDRESS: &str = "127.0.0.1:3001";

#[tokio::main]
async fn main() {
    tokio::spawn(server());
    // Give the server time to start. You don't actually need this!
    sleep(Duration::from_secs_f32(0.25)).await;
    client().await;
}

async fn server() {
    let listener = TcpListener::bind(ADDRESS)
        .await
        .expect("Failed to bind to address");
    println!("server started");
    loop {
        let (mut stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            while let Ok(n) = stream.read(buf.as_mut()).await {
                if n == 0 {
                    return; // Connection closed
                }
                stream
                    .write_all(&buf[..n])
                    .await
                    .expect("Failed to write to stream");
            }
        });
    }
}

async fn client() {
    let mut stream = TcpStream::connect(ADDRESS)
        .await
        .expect("Failed to connect");

    let input = spawn_blocking(|| {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer
    })
    .await
    .unwrap();

    stream
        .write_all(input.as_bytes())
        .await
        .expect("Failed to write");

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.expect("Failed to read");
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
}
