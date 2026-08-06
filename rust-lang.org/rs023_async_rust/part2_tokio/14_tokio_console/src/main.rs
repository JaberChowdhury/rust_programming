// CONCEPT: Set up tokio-console for runtime inspection.

#[tokio::main]
async fn main() {
    // Note: requires `tokio_unstable` cfg for task builder
    console_subscriber::init();

    let task1 = tokio::task::Builder::new()
        .name("worker_1")
        .spawn(async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        })
        .unwrap();

    let task2 = tokio::task::Builder::new()
        .name("starved_task")
        .spawn(async {
            loop {
                // Not yielding properly
                let mut x: u64 = 0;
                for i in 0..10_000 {
                    x += i;
                }
                tokio::task::yield_now().await;
            }
        })
        .unwrap();

    let _ = tokio::join!(task1, task2);
}
// EXERCISE:
// 1. Run tokio-console in your terminal and view these tasks.
