use tokio::sync::broadcast;

pub async fn run() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tx.send("Hello Subscribers!".to_string()).unwrap();

    println!("rx1: {}", rx1.recv().await.unwrap());
    println!("rx2: {}", rx2.recv().await.unwrap());
}
