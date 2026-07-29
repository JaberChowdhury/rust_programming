// CONCEPT: Async-safe shared state.
// WHY: standard library sync primitives (like Mutex) block the OS thread. In async, we must use primitives that yield to the runtime.

use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, Semaphore, Barrier};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 1. Mutex - Shared counter
    println!("--- tokio::sync::Mutex ---");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..100 {
        let c = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            let mut lock = c.lock().await;
            *lock += 1;
        }));
    }
    for h in handles { h.await.unwrap(); }
    println!("Final counter: {}", *counter.lock().await);

    // 2. RwLock - Many readers, few writers
    println!("\n--- tokio::sync::RwLock ---");
    let cache = Arc::new(RwLock::new("initial".to_string()));
    let read_lock = cache.read().await;
    println!("Read: {}", *read_lock);
    drop(read_lock);
    
    let mut write_lock = cache.write().await;
    *write_lock = "updated".to_string();
    println!("Wrote to cache");
    drop(write_lock);

    // 3. Semaphore - Limit concurrency
    println!("\n--- tokio::sync::Semaphore ---");
    let sem = Arc::new(Semaphore::new(2)); // Only 2 concurrent tasks
    let mut handles = vec![];
    for i in 0..4 {
        let s = Arc::clone(&sem);
        handles.push(tokio::spawn(async move {
            let _permit = s.acquire().await.unwrap();
            println!("Task {} acquired permit", i);
            sleep(Duration::from_millis(50)).await;
            println!("Task {} releasing permit", i);
        }));
    }
    for h in handles { h.await.unwrap(); }

    // 4. Barrier - Coordinate startup
    println!("\n--- tokio::sync::Barrier ---");
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    for i in 0..3 {
        let b = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            println!("Task {} waiting at barrier", i);
            b.wait().await;
            println!("Task {} crossed barrier", i);
        }));
    }
    for h in handles { h.await.unwrap(); }

    // DEADLOCK WARNING:
    // Using std::sync::Mutex across an .await point can deadlock the entire worker thread!
    // let std_mu = std::sync::Mutex::new(0);
    // let _guard = std_mu.lock().unwrap();
    // sleep(...).await; // <-- BAD! Thread is blocked, but task yielded.
}

// EXERCISE:
// 1. Change the semaphore permits to 1 and observe the sequential execution output.
// 2. Try holding the RwLock `read_lock` while acquiring the `write_lock` and see it hang.
