# 12_sync_primitives

## 🎯 Concept

This exercise demonstrates Tokio's asynchronous synchronization primitives. These are drop-in replacements for standard library primitives, but they are safe to hold across `.await` points because they yield to the runtime instead of blocking the OS thread.

## 🧠 Key Ideas

- `Mutex`: Exclusive access to data.
- `RwLock`: Allows multiple concurrent readers or one exclusive writer.
- `Semaphore`: Limits the number of concurrent operations (great for connection pools or rate limiting).
- `Barrier`: Ensures multiple tasks all reach a certain point before any of them proceed.
- Never hold a `std::sync::Mutex` across an `.await` point.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | Async runtime and `tokio::sync` primitives. |

## ▶️ How to Run

```bash
cargo run -p p1_12_sync_primitives
```

## 👀 What to Observe

You will see the Mutex safely incremented to 100. The Semaphore will explicitly limit the execution to batches of 2 tasks at a time. The Barrier will force all 3 tasks to wait before crossing simultaneously.

## 🔗 How This Connects

Sometimes tasks get stuck waiting on locks or external services. In `13_timeouts_cancellation`, we'll learn how to forcibly stop tasks that are taking too long.

## 🏋️ Your Turn

- Alter the semaphore demo to spawn 10 tasks with 5 permits.
- Create a deliberate deadlock using `std::sync::Mutex` across an await (be prepared to `Ctrl+C`).
