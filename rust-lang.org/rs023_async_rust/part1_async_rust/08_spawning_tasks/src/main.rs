// CONCEPT: Understand `tokio::spawn` and task lifecycle.
// WHY: Spawning allows background concurrent execution. Tasks must be `'static + Send`.

use std::sync::Arc;

async fn do_work(id: u32, shared: Arc<String>) -> u32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    println!("Task {} working on: {}", id, shared);
    id * 2
}

#[tokio::main]
async fn main() {
    let shared_data = Arc::new("Important Config".to_string());
    let mut handles = vec![];

    println!("Spawning 10 tasks...");
    for i in 0..10 {
        let data_clone = Arc::clone(&shared_data);
        
        // Spawn requires 'static, so we move owned data (or Arc clones) into it.
        let handle = tokio::spawn(async move {
            do_work(i, data_clone).await
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    println!("All tasks finished. Results: {:?}", results);

    // Cancellation demo
    let slow_task = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        println!("This should not print!");
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    slow_task.abort(); // Cancel the task
    let res = slow_task.await;
    println!("Slow task result after abort: {:?}", res);
    
    // THIS WONT COMPILE: Capturing a non-Send type
    // let rc = std::rc::Rc::new(5);
    // tokio::spawn(async move {
    //     println!("{}", rc);
    // });
}

// EXERCISE:
// 1. Uncomment the `Rc` code block and read the compiler error regarding `Send`.
// 2. Change the sleep in the cancellation demo so it finishes before `abort()` is called.
