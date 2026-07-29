# 03_future_trait

## 🎯 Concept

This exercise demonstrates the internal mechanics of Rust's `Future` trait by manually implementing futures and a rudimentary task executor, without relying on Tokio.

## 🧠 Key Ideas

- A `Future` is a state machine that must be polled to make progress.
- `poll` returns either `Poll::Ready(value)` or `Poll::Pending`.
- If a future returns `Pending`, it is responsible for using the `Waker` from the `Context` to notify the executor when it can make progress again.
- An executor repeatedly polls futures until they are `Ready`.

## 📦 Dependencies

*(None required for this exercise)*

## ▶️ How to Run

```bash
cargo run -p p1_03_future_trait
```

## 👀 What to Observe

You will see the polling statements printed to the console. The `PendingOnceFuture` will be polled twice: once returning `Pending` (and re-queueing itself via the waker), and once returning `Ready`.

## 🔗 How This Connects

Now that we understand how Futures work under the hood, the next exercise (`04_async_await_syntax`) will explore how Rust's `async`/`await` syntax provides a convenient abstraction over these state machines.

## 🏋️ Your Turn

- Implement a custom future that requires 3 polls to finish.
- Modify the executor to print out the size of the task queue in each iteration.
