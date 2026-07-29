use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};

pub async fn run_client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Client connected to server");

    let (read, mut write) = stream.split();
    let mut reader = BufReader::new(read);

    for i in 0..5 {
        let msg = format!("Message {}\n", i);
        write.write_all(msg.as_bytes()).await?;
        
        let mut response = String::new();
        reader.read_line(&mut response).await?;
        println!("Client received: {}", response.trim());
    }

    Ok(())
}
