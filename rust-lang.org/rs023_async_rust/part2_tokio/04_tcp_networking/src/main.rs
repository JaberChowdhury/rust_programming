// CONCEPT: Build a working TCP echo server and client.

mod client;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Spawn server in background
    tokio::spawn(async {
        server::run_server().await.unwrap();
    });

    // Give server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Run client
    client::run_client().await?;

    Ok(())
}
// EXERCISE:
// 1. Track connected clients in a shared Vec.
