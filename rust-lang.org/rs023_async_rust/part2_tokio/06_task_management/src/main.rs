// CONCEPT: Master spawn, spawn_blocking, spawn_local, JoinHandle.

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    // Spawn async tasks
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            println!(
                "Task {} running on thread {:?}",
                i,
                std::thread::current().id()
            );
            i * 2
        });
        handles.push(handle);
    }

    // Spawn blocking for CPU-heavy
    let blocking_handle = tokio::task::spawn_blocking(|| {
        println!("Blocking task on thread {:?}", std::thread::current().id());
        let mut sum: u64 = 0;
        for i in 0..10_000_000 {
            sum += i;
        }
        sum
    });

    for h in handles {
        let res = h.await.unwrap();
        println!("Async task returned {}", res);
    }

    let b_res = blocking_handle.await.unwrap();
    println!("Blocking task returned {}", b_res);

    // Cancel
    let abort_handle = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    });
    abort_handle.abort();
    println!("Aborted long running task");
}
// EXERCISE:
// 1. Measure time taken by blocking task.
