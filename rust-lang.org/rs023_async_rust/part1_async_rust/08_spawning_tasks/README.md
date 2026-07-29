# 08_spawning_tasks

## 🎯 Concept

This exercise demonstrates how to spawn concurrent tasks using `tokio::spawn`, how to share state across them using `Arc`, and how to manually cancel tasks.

## 🧠 Key Ideas

- `tokio::spawn` requires the future to be `'static + Send`.
- `'static` means the future cannot borrow local variables from the spawner; you must use `move`.
- `Send` means the data captured by the future must be safe to move between threads.
- Dropping a `JoinHandle` *detaches* the task; calling `.abort()` explicitly cancels it.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | For the async runtime, `spawn`, and `sleep`. |

## ▶️ How to Run

```bash
cargo run -p p1_08_spawning_tasks
```

## 👀 What to Observe

You will see 10 tasks executing concurrently and printing their output. At the end, you'll see that the slow task's print statement never executes because it was aborted, resulting in a cancelled `JoinError`.

## 🔗 How This Connects

Spawning is great for background work. But what if you want to wait for multiple tasks dynamically? The next exercise, `09_join_select`, covers macros for grouping and racing futures.

## 🏋️ Your Turn

- Try passing a standard reference (`&String`) into the spawned task instead of an `Arc` and observe the `'static` lifetime error.
