# 01_why_async

## 🎯 Concept

This exercise demonstrates the fundamental difference between blocking and non-blocking I/O. By simulating network requests that take 200ms each, we can see how asynchronous execution allows multiple tasks to run concurrently without waiting for each one to finish sequentially.

## 🧠 Key Ideas

- Blocking operations halt the current thread until completion.
- Non-blocking (async) operations allow the thread to yield control and perform other work while waiting.
- Concurrency enables handling many I/O operations simultaneously, significantly reducing total execution time.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | Provides the async runtime and non-blocking `sleep` utility. |

## ▶️ How to Run

```bash
cargo run -p p1_01_why_async
```

## 👀 What to Observe

You should observe that the sequential execution takes approximately 1000ms (5 * 200ms), while the asynchronous concurrent execution takes only about 200ms, as all 5 simulated requests are waiting simultaneously.

## 🔗 How This Connects

This exercise establishes the basic motivation for async programming. In the next exercise (`02_threads_vs_async`), we will see how async tasks compare to OS threads in terms of resource usage.

## 🏋️ Your Turn

- Change the number of simulated requests from 5 to 50.
- Try replacing `tokio::time::sleep` with `std::thread::sleep` inside the `async_request` function and observe the impact on execution time.
