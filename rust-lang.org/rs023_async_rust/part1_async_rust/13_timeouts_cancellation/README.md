# 13_timeouts_cancellation

## 🎯 Concept

This exercise focuses on resilience by demonstrating how to apply timeouts, handle task cancellation cooperatively using `select!`, and implement an exponential backoff retry loop.

## 🧠 Key Ideas

- `tokio::time::timeout` wraps any future and cancels it if it doesn't complete within the duration.
- Async tasks in Rust use *cooperative cancellation*; when a future is dropped, it simply stops being polled.
- `select!` is perfect for listening to a "cancel" signal alongside performing work.
- Retrying with exponential backoff prevents thundering herd problems when a service is struggling.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | Async runtime and `tokio::time` utilities. |

## ▶️ How to Run

```bash
cargo run -p p1_13_timeouts_cancellation
```

## 👀 What to Observe

You will see the database query get aggressively cut off by the timeout. Then, you'll see a background task cleanly cancelled via a oneshot signal. Finally, a retry loop will fail 3 times, backing off exponentially.

## 🔗 How This Connects

Timeouts are often applied to collections of tasks. In `14_joinset`, we will see how to manage and collect results from many dynamically spawned tasks securely.

## 🏋️ Your Turn

- Write a function that races a `timeout` against a `tokio::signal::ctrl_c()`.
