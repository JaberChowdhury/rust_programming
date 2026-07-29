// CONCEPT: Handle all task failure modes.

use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    set.spawn(async { Ok::<&str, anyhow::Error>("Success") });
    set.spawn(async { panic!("Simulated panic!") });

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(val)) => println!("Task success: {}", val),
            Ok(Err(e)) => println!("Task error: {}", e),
            Err(e) if e.is_panic() => println!("Task panicked!"),
            Err(e) if e.is_cancelled() => println!("Task cancelled!"),
            Err(_) => println!("Task failed with unknown error"),
        }
    }
}
// EXERCISE:
// 1. Create a restart loop for panicked tasks.
