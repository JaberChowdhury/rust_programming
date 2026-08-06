// CONCEPT: All four Tokio channel types with real use cases.
// WHY: Channels are the standard way to communicate between async tasks without sharing memory.

use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 1. mpsc (Multi-Producer, Single-Consumer) - Work queue
    println!("--- MPSC Channel (Work Queue) ---");
    let (tx, mut rx) = mpsc::channel(32);
    for i in 0..3 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            tx_clone.send(format!("Job {}", i)).await.unwrap();
        });
    }
    drop(tx); // Close the original sender so the receiver knows when to stop
    while let Some(msg) = rx.recv().await {
        println!("Processed: {}", msg);
    }

    // 2. oneshot - Request/Response
    println!("\n--- Oneshot Channel (Request/Response) ---");
    let (req_tx, req_rx) = oneshot::channel();
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        req_tx.send("Data from actor").unwrap();
    });
    let resp = req_rx.await.unwrap();
    println!("Received oneshot: {}", resp);

    // 3. broadcast - Chat room (Multi-Producer, Multi-Consumer)
    println!("\n--- Broadcast Channel (Pub/Sub) ---");
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    tx.send("System going down!").unwrap();
    println!("Subscriber 1 received: {}", rx1.recv().await.unwrap());
    println!("Subscriber 2 received: {}", rx2.recv().await.unwrap());

    // 4. watch - Config watcher (Single-Producer, Multi-Consumer, only keeps latest value)
    println!("\n--- Watch Channel (Config reload) ---");
    let (tx, mut rx) = watch::channel("v1");
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        tx.send("v2").unwrap();
    });
    // Wait for a change
    rx.changed().await.unwrap();
    println!("Config updated to: {}", *rx.borrow());
}

// EXERCISE:
// 1. Try sending a message on `oneshot` after the receiver is dropped and handle the error.
// 2. Send 3 messages rapidly on the `watch` channel before yielding. See what the receiver gets.
