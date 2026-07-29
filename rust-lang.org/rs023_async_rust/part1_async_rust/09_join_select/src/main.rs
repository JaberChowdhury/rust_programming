// CONCEPT: Master concurrent future execution.
// WHY: We often need to wait for multiple operations to finish, or race them and take the first successful one.

use std::time::Duration;
use tokio::time::sleep;

async fn api_call(name: &str, ms: u64, fail: bool) -> Result<String, String> {
    sleep(Duration::from_millis(ms)).await;
    if fail {
        Err(format!("{} failed!", name))
    } else {
        Ok(format!("{} done", name))
    }
}

#[tokio::main]
async fn main() {
    // 1. tokio::join! - Wait for all (returns a tuple)
    println!("--- tokio::join! ---");
    let start = std::time::Instant::now();
    let (r1, r2, r3) = tokio::join!(
        api_call("A", 200, false),
        api_call("B", 100, false),
        api_call("C", 150, false)
    );
    println!("Join results: {:?}, {:?}, {:?}", r1, r2, r3);
    println!("Elapsed: {:?} (Expected ~200ms)\n", start.elapsed());

    // 2. tokio::try_join! - Short circuit on error
    println!("--- tokio::try_join! ---");
    let start = std::time::Instant::now();
    let res = tokio::try_join!(
        api_call("D", 300, false),
        api_call("E", 100, true), // This fails early
        api_call("F", 500, false)
    );
    println!("TryJoin result: {:?}", res);
    println!("Elapsed: {:?} (Expected ~100ms)\n", start.elapsed());

    // 3. tokio::select! - Race futures
    println!("--- tokio::select! ---");
    let start = std::time::Instant::now();
    tokio::select! {
        val = api_call("Slow", 500, false) => println!("Slow finished: {:?}", val),
        val = api_call("Fast", 100, false) => println!("Fast finished: {:?}", val),
    }
    println!("Elapsed: {:?} (Expected ~100ms)\n", start.elapsed());

    // 4. futures::future::join_all - Dynamic number of futures
    println!("--- futures::future::join_all ---");
    let mut tasks = vec![];
    for _ in 0..5 {
        tasks.push(api_call("Dynamic", 150, false));
    }
    let results = futures::future::join_all(tasks).await;
    println!("JoinAll results: {:?}", results);
}

// EXERCISE:
// 1. Change the failing `try_join` task to succeed and observe the new elapsed time.
// 2. Add a third arm to `select!` that completes in 50ms and see who wins.
