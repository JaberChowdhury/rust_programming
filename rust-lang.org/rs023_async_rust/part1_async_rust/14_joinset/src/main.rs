// CONCEPT: Structured concurrency with `JoinSet`.
// WHY: Managing a Vec of JoinHandles is tedious and doesn't handle dynamic spawning/reaping well. JoinSet solves this.

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

async fn scrape_url(id: usize, fail: bool) -> Result<String, String> {
    sleep(Duration::from_millis(100)).await;
    if fail {
        Err(format!("Failed to scrape URL {}", id))
    } else {
        Ok(format!("Data from URL {}", id))
    }
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    let sem = Arc::new(Semaphore::new(5)); // Max 5 concurrent scrapes

    println!("Spawning 20 tasks into JoinSet...");
    for i in 0..20 {
        let s = Arc::clone(&sem);
        // We push tasks into the set. It handles spawning them.
        set.spawn(async move {
            let _permit = s.acquire().await.unwrap();
            let fail = i % 7 == 0; // Arbitrary failure condition
            scrape_url(i, fail).await
        });
    }

    let mut successes = 0;
    let mut failures = 0;

    // join_next() returns results as soon as ANY task finishes
    // Note: They do not come back in the order they were spawned!
    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(data)) => successes += 1,
            Ok(Err(e)) => {
                println!("Logic error: {}", e);
                failures += 1;
            }
            Err(e) => {
                println!("Task crashed/cancelled: {}", e);
                failures += 1;
            }
        }
    }

    println!(
        "Finished processing! Successes: {}, Failures: {}",
        successes, failures
    );

    // Abort all demo
    println!("\n--- Abort all demo ---");
    let mut abort_set = JoinSet::new();
    for i in 0..10 {
        abort_set.spawn(async move {
            sleep(Duration::from_secs(10)).await;
            println!("Task {} finished", i);
        });
    }

    sleep(Duration::from_millis(50)).await;
    abort_set.abort_all();
    println!("Aborted all tasks in the set. Awaiting completion...");
    while let Some(res) = abort_set.join_next().await {
        assert!(res.is_err()); // JoinError::Cancelled
    }
    println!("All tasks cleaned up safely.");
}

// EXERCISE:
// 1. Remove the Semaphore and observe how it runs without concurrency limits.
// 2. Change `join_next` to a loop that collects the first 3 successful results, then calls `abort_all()`.
