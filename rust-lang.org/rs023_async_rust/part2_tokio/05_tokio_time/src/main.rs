// CONCEPT: All Tokio time utilities.

use tokio::time::{sleep, interval, timeout, Instant, Duration};

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // Sleep
    println!("Sleeping...");
    sleep(Duration::from_millis(200)).await;
    println!("Slept for {:?}", start.elapsed());

    // Timeout
    let slow_operation = sleep(Duration::from_secs(2));
    match timeout(Duration::from_secs(1), slow_operation).await {
        Ok(_) => println!("Operation finished"),
        Err(_) => println!("Operation timed out!"),
    }

    // Interval (Heartbeat)
    let mut ticker = interval(Duration::from_millis(500));
    for i in 0..5 {
        ticker.tick().await;
        println!("Heartbeat {}", i);
    }
}
// EXERCISE:
// 1. Test missed interval ticks (block the thread).
