// CONCEPT: Compare different runtime configurations.
// WHY: Tokio provides different runtime flavors (current_thread vs multi_thread). Choosing the right one is important for performance.

use std::thread;

fn print_thread_id(prefix: &str) {
    println!("{}: running on thread {:?}", prefix, thread::current().id());
}

fn main() {
    // 1. current_thread runtime (single thread)
    println!("--- Current Thread Runtime ---");
    let rt_current = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt_current.block_on(async {
        print_thread_id("Task 1 (CurrentThread)");
        tokio::spawn(async {
            print_thread_id("Task 2 (CurrentThread)");
        })
        .await
        .unwrap();
    });

    // 2. multi_thread runtime (default with N workers)
    println!("\n--- Multi Thread Runtime (2 workers) ---");
    let rt_multi = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt_multi.block_on(async {
        print_thread_id("Task 1 (MultiThread)");
        let t1 = tokio::spawn(async {
            print_thread_id("Spawned A (MultiThread)");
        });
        let t2 = tokio::spawn(async {
            print_thread_id("Spawned B (MultiThread)");
        });
        let _ = tokio::join!(t1, t2);
    });
}

// EXERCISE:
// 1. Change `worker_threads` to 1 and observe the thread IDs.
// 2. Change `main` to use `#[tokio::main(flavor = "current_thread")]` instead of building manually (you'll need to remove the manual builder code).
