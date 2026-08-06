// CONCEPT: Runtime internals and configuration.

use tokio::runtime::Builder;

fn main() {
    // WHY: Building manually gives more control over thread names, hooks, and worker threads.
    let rt = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("my-custom-name")
        .on_thread_start(|| {
            println!("Thread started");
        })
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        println!("Hello from manual runtime!");
        tokio::spawn(async {
            println!("Spawned task!");
        })
        .await
        .unwrap();
    });
}
// EXERCISE:
// 1. Change worker_threads to 1.
// 2. Use Builder::new_current_thread().
