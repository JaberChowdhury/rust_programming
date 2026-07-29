// CONCEPT: Demonstrate threads vs async tasks for concurrent work.
// WHY: To show that async tasks are significantly lighter than OS threads in terms of memory and creation overhead.

use std::time::Instant;

#[tokio::main]
async fn main() {
    println!("Size of OS thread handle: {} bytes", std::mem::size_of::<std::thread::JoinHandle<()>>());
    println!("Size of Tokio task handle: {} bytes", std::mem::size_of::<tokio::task::JoinHandle<()>>());

    let count = 1000;

    println!("\n--- Spawning {} OS threads ---", count);
    let start = Instant::now();
    let mut thread_handles = Vec::with_capacity(count);
    for _ in 0..count {
        thread_handles.push(std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }));
    }
    for handle in thread_handles {
        let _ = handle.join();
    }
    println!("OS threads total time: {:?}", start.elapsed());

    println!("\n--- Spawning {} Async tasks ---", count);
    let start = Instant::now();
    let mut task_handles = Vec::with_capacity(count);
    for _ in 0..count {
        task_handles.push(tokio::spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }));
    }
    for handle in task_handles {
        let _ = handle.await;
    }
    println!("Async tasks total time: {:?}", start.elapsed());
    
    println!("\n--- spawn_blocking demo ---");
    let blocking_result = tokio::task::spawn_blocking(|| {
        // CPU-bound work
        let mut sum = 0;
        for i in 0..10_000_000 {
            sum += i;
        }
        sum
    }).await.unwrap();
    println!("Result of blocking task: {}", blocking_result);
}

// EXERCISE:
// 1. Increase `count` to 10_000 (warning: OS threads might crash or take very long depending on your OS limits).
// 2. Print out `std::thread::current().id()` inside both the thread and async task loops to see thread utilization.
