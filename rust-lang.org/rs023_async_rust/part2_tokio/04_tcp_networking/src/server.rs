use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub async fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");

    let conn_count = Arc::new(AtomicUsize::new(0));

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        let count = Arc::clone(&conn_count);
        count.fetch_add(1, Ordering::SeqCst);
        println!("Total connections: {}", count.load(Ordering::SeqCst));

        tokio::spawn(async move {
            let (read, mut write) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                line.clear();
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                println!("Server received: {}", line.trim());
                write.write_all(line.as_bytes()).await.unwrap();
            }
            println!("Connection closed");
        });
    }
}
