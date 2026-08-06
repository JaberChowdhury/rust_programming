// CONCEPT: Understand Tokio performance characteristics.

use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut handles = Vec::with_capacity(10_000);

    for _ in 0..10_000 {
        handles.push(tokio::spawn(async { 1 }));
    }

    for h in handles {
        let _ = h.await.unwrap();
    }

    println!(
        "Spawning and awaiting 10k tasks took: {:?}",
        start.elapsed()
    );
}
// EXERCISE:
// 1. Run in --release mode and observe the speedup.
