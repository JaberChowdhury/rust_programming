# 06_executors_runtimes

## 🎯 Concept

This exercise demonstrates the different runtime configurations available in Tokio: the single-threaded `current_thread` runtime and the multi-threaded `multi_thread` runtime.

## 🧠 Key Ideas

- `current_thread` runs all tasks concurrently on the same OS thread. It has less synchronization overhead and is great for lightweight apps or environments with a single core.
- `multi_thread` distributes tasks across a thread pool using a work-stealing scheduler.
- You can manually configure the runtime using `tokio::runtime::Builder` or rely on the `#[tokio::main]` macro.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | For the async runtime and `Builder`. |

## ▶️ How to Run

```bash
cargo run -p p1_06_executors_runtimes
```

## 👀 What to Observe

You will see that tasks executed in the `current_thread` runtime all share the same OS thread ID. In the `multi_thread` runtime, the tasks will likely execute on different thread IDs.

## 🔗 How This Connects

Now that you know how the runtime executes tasks, next we will learn how to properly handle errors in those tasks in `07_async_error_handling`.

## 🏋️ Your Turn

- Try using `#[tokio::main(flavor = "current_thread")]` to configure the main function instead of the builder.
