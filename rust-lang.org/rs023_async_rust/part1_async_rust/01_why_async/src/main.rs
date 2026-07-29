// CONCEPT: Show the difference between blocking and non-blocking I/O through simulation.
// WHY: We use `std::thread::sleep` for blocking and `tokio::time::sleep` for non-blocking to clearly show the time difference when running concurrently.

use std::time::Instant;

fn blocking_request(id: usize) {
    std::thread::sleep(std::time::Duration::from_millis(200));
    println!("Blocking request {} done", id);
}

async fn async_request(id: usize) {
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    println!("Async request {} done", id);
}

#[tokio::main]
async fn main() {
    println!("--- Version A: Sequential (Blocking) ---");
    let start = Instant::now();
    for i in 1..=5 {
        blocking_request(i);
    }
    println!("Sequential total time: {:?}", start.elapsed()); // Expected ~1000ms

    println!("\n--- Version B: Concurrent (Async) ---");
    let start = Instant::now();
    let mut handles = Vec::new();
    for i in 1..=5 {
        handles.push(tokio::spawn(async move {
            async_request(i).await;
        }));
    }
    for handle in handles {
        let _ = handle.await;
    }
    println!("Async total time: {:?}", start.elapsed()); // Expected ~200ms
}

// EXERCISE:
// 1. Change the number of requests to 10 and observe the time difference.
// 2. Try removing the `tokio::spawn` and just `.await`ing the `async_request` in the loop. What happens to the concurrent version?
