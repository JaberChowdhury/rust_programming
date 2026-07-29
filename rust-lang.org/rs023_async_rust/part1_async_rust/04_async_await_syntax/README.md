# 04_async_await_syntax

## 🎯 Concept

This exercise explains Rust's `async`/`await` syntax, showing that async functions are actually syntactic sugar for returning a `Future`, and demonstrating that Futures are lazy.

## 🧠 Key Ideas

- `async fn` desugars to a function returning an `impl Future`.
- Futures in Rust are **lazy**; they do absolutely nothing until they are `.await`ed (or polled).
- `.await` is used to pause the current async execution context until the awaited Future is ready.
- You can create arbitrary async contexts using `async {}` blocks.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | Provides the async runtime (`#[tokio::main]`) to execute the futures. |

## ▶️ How to Run

```bash
cargo run -p p1_04_async_await_syntax
```

## 👀 What to Observe

Notice the print statement proving that calling an async function doesn't execute its body until `.await` is invoked.

## 🔗 How This Connects

After learning the syntax, we will next tackle one of Rust's more complex async features: `Pin` and self-referential structures in `05_pin_unpin`.

## 🏋️ Your Turn

- Write a function that returns a `Result<i32, String>` and handle it after awaiting.
- See what the compiler says if you try to pass an un-awaited future to a function expecting a `String`.
