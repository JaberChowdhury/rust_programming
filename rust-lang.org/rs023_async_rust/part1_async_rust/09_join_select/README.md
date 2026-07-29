# 09_join_select

## 🎯 Concept

This exercise explores how to manage multiple futures concurrently on a single task using macros like `join!`, `try_join!`, and `select!`.

## 🧠 Key Ideas

- `join!` runs multiple futures concurrently and waits for all of them to finish.
- `try_join!` is like `join!` but returns early if *any* of the futures return an `Err`.
- `select!` runs multiple futures and returns as soon as the *first* one finishes, dropping (cancelling) the rest.
- `futures::future::join_all` is useful when you have a dynamic collection of futures (like a `Vec`) rather than a static number.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | For the async runtime, `join!`, `try_join!`, and `select!`. |
| futures | For `join_all` utility. |

## ▶️ How to Run

```bash
cargo run -p p1_09_join_select
```

## 👀 What to Observe

Notice the elapsed times for each block. `join!` takes the time of the longest task. `try_join!` aborts early and takes the time of the task that failed fastest. `select!` takes the time of the fastest task.

## 🔗 How This Connects

While `join_all` handles dynamic collections of futures, it collects everything into memory. Next, in `10_async_streams`, we'll see how to yield dynamic values asynchronously over time without buffering them all at once.

## 🏋️ Your Turn

- Alter `try_join!` so the failure happens last, and see how long it takes.
- Add a timeout future to the `select!` block.
