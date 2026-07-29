// CONCEPT: Production-grade shutdown handling.

use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(1);
    
    let worker1 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = rx1.recv() => {
                    println!("Worker 1 shutting down gracefully");
                    break;
                }
                _ = sleep(Duration::from_millis(500)) => {
                    println!("Worker 1 doing work...");
                }
            }
        }
    });

    println!("Press Ctrl+C to shutdown...");
    tokio::signal::ctrl_c().await.unwrap();
    println!("Shutdown signal received");
    
    let _ = tx.send(());
    
    let _ = tokio::time::timeout(Duration::from_secs(2), worker1).await;
    println!("All workers exited");
}
// EXERCISE:
// 1. Have multiple workers listen for shutdown.
