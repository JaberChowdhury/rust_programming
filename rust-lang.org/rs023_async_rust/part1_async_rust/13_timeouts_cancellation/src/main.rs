// CONCEPT: Handle slow operations gracefully.
// WHY: In production, external services fail or hang. We must put timeouts on everything to prevent resource exhaustion.

use tokio::time::{sleep, timeout, Duration};

async fn slow_db_query() -> String {
    sleep(Duration::from_millis(500)).await;
    "Database result".to_string()
}

// Deadline propagation helper
async fn with_deadline<F, T>(deadline: Duration, fut: F) -> Result<T, ()> 
where 
    F: std::future::Future<Output = T> 
{
    match timeout(deadline, fut).await {
        Ok(val) => Ok(val),
        Err(_) => Err(()),
    }
}

#[tokio::main]
async fn main() {
    // 1. tokio::time::timeout
    println!("--- Timeout Demo ---");
    let result = timeout(Duration::from_millis(200), slow_db_query()).await;
    match result {
        Ok(data) => println!("Success: {}", data),
        Err(_) => println!("Error: Query timed out!"),
    }

    // 2. select! for cancellation via signal
    println!("\n--- select! Cancellation ---");
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    
    let work_task = tokio::spawn(async move {
        tokio::select! {
            _ = slow_db_query() => println!("Work completed normally"),
            _ = rx => println!("Work was cancelled by signal!"),
        }
    });
    
    sleep(Duration::from_millis(100)).await;
    tx.send(()).unwrap();
    work_task.await.unwrap();

    // 3. Retry with backoff
    println!("\n--- Retry with Backoff ---");
    let mut retries = 3;
    let mut delay = 50;
    while retries > 0 {
        match timeout(Duration::from_millis(10), slow_db_query()).await {
            Ok(res) => {
                println!("Success: {}", res);
                break;
            }
            Err(_) => {
                println!("Timeout, retrying in {}ms...", delay);
                sleep(Duration::from_millis(delay)).await;
                delay *= 2; // Exponential backoff
                retries -= 1;
            }
        }
    }
}

// EXERCISE:
// 1. Change the timeout on `slow_db_query` to 600ms and verify it succeeds.
// 2. Modify the backoff loop to add a small amount of random jitter to the delay.
