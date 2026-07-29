# 02_threads_vs_async

## 🎯 Concept

This exercise compares the overhead of spawning OS threads versus spawning asynchronous tasks. It demonstrates that async tasks are lightweight and more efficient for highly concurrent I/O-bound workloads.

## 🧠 Key Ideas

- OS threads are relatively heavy, requiring significant memory (stack space) and OS-level context switching.
- Async tasks (green threads) are lightweight, managed by the runtime (Tokio) entirely in user space.
- `spawn_blocking` should be used for CPU-bound work to prevent blocking the async runtime's worker threads.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | For async tasks, `sleep`, and `spawn_blocking`. |

## ▶️ How to Run

```bash
cargo run -p p1_02_threads_vs_async
```

## 👀 What to Observe

Both approaches will take roughly 1 second to execute (because tasks run concurrently), but you may notice slightly more overhead (wall clock time) for thread creation compared to async task creation. You'll also see the memory size difference of their respective handles.

## 🔗 How This Connects

Following the introduction to non-blocking I/O in the previous exercise, this highlights the scalability benefits. Next, in `03_future_trait`, we will look under the hood at how async works without Tokio.

## 🏋️ Your Turn

- Try increasing the `count` to 10,000. Depending on your system limits, spawning 10,000 OS threads may fail or consume excessive memory, while async tasks will handle it effortlessly.
