use tokio::sync::oneshot;

pub async fn run() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("One-time result").unwrap();
    });

    let res = rx.await.unwrap();
    println!("oneshot received: {}", res);
}
