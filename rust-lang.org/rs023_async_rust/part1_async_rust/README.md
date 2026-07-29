# Part 1: Async Rust Foundations

This section covers the core concepts of asynchronous programming in Rust, moving from the foundational `Future` trait up to high-level concurrency management using Tokio.

## 📚 Prerequisites

- Basic understanding of standard Rust (Ownership, Traits, Enums)
- The Rust standard library

## 🚀 Exercises

| Exercise | Description |
| -------- | ----------- |
| `01_why_async` | The difference between blocking and non-blocking I/O. |
| `02_threads_vs_async` | Comparing OS threads vs async tasks in overhead. |
| `03_future_trait` | Manually implementing `Future` and a basic Executor. |
| `04_async_await_syntax` | Understanding how `async` and `.await` desugar. |
| `05_pin_unpin` | Why `Pin` is necessary for self-referential futures. |
| `06_executors_runtimes` | Exploring `current_thread` vs `multi_thread` runtimes. |
| `07_async_error_handling` | Propagating and handling errors across task boundaries. |
| `08_spawning_tasks` | Background execution and `'static + Send` bounds. |
| `09_join_select` | Concurrent execution with `join!`, `try_join!`, and `select!`. |
| `10_async_streams` | Processing streams of async data over time. |
| `11_async_channels` | Task communication via mpsc, oneshot, broadcast, and watch. |
| `12_sync_primitives` | Yielding locks: Mutex, RwLock, Semaphore, Barrier. |
| `13_timeouts_cancellation` | Handling long-running operations and backoff loops. |
| `14_joinset` | Structured concurrency and mass task management. |
| `15_custom_future` | Building real `Future` implementations from scratch. |

## ✅ Key Concepts Checklist

- [ ] I understand why async is beneficial for I/O bound work.
- [ ] I understand the difference between `Poll::Ready` and `Poll::Pending`.
- [ ] I understand that Futures do nothing unless polled (`.await`ed).
- [ ] I know how to spawn a task and wait for it.
- [ ] I know how to race multiple futures using `select!`.
- [ ] I understand which channel to use for which scenario.
