use tokio::sync::watch;

pub async fn run() {
    let (tx, mut rx) = watch::channel("Initial config");

    tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("Config changed to: {}", *rx.borrow());
        }
    });

    tx.send("New Config").unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
}
