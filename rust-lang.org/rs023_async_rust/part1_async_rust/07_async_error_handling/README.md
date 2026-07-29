# 07_async_error_handling

## 🎯 Concept

This exercise demonstrates how to handle and propagate errors in async Rust using the `?` operator, `thiserror`, and `anyhow`. It also covers the "double Result" returned when awaiting a spawned task.

## 🧠 Key Ideas

- The `?` operator works perfectly in `async fn`.
- `thiserror` is great for library-level custom errors.
- `anyhow` is ideal for application-level error handling, allowing you to add context.
- When you `await` a `tokio::spawn` handle, you get a `Result<T, JoinError>`. If the task itself returns a `Result`, you end up with a nested `Result`.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | For the async runtime and spawning. |
| anyhow | Easy error handling and context application. |
| thiserror | Deriving standard `Error` traits for custom enums. |

## ▶️ How to Run

```bash
cargo run -p p1_07_async_error_handling
```

## 👀 What to Observe

You will see the successful execution, followed by the anyhow trace showing the custom error wrapped in context. Finally, you will see how a task's internal error is extracted from the `JoinHandle`.

## 🔗 How This Connects

Proper error handling is vital before managing multiple tasks. In `08_spawning_tasks`, we will delve deeper into task lifecycles and data sharing.

## 🏋️ Your Turn

- Add a new variant to `MyError` and return it.
- Modify the spawned task to `panic!` and see how `JoinError` handles it.
