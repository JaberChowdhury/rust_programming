use tokio::sync::mpsc;

pub async fn run() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("mpsc received: {}", msg);
    }
}
