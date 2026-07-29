# 05_pin_unpin

## 🎯 Concept

This exercise explains why `Pin` and `Unpin` exist. Many generated Futures contain references to their own data. If they are moved in memory, those references become invalid. `Pin` provides a guarantee that the underlying memory will not be moved.

## 🧠 Key Ideas

- Self-referential structures are dangerous in Rust because moving the struct invalidates internal pointers.
- Async blocks generate state machines that are often self-referential.
- `Pin<P>` ensures the pointee cannot be moved.
- Most types implement `Unpin`, meaning they don't care if they are moved. Futures often do not implement `Unpin` (`!Unpin`).

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | For the async runtime and `tokio::pin!`. |
| tokio-stream | To demonstrate an API (`StreamExt::next`) that requires a pinned reference. |

## ▶️ How to Run

```bash
cargo run -p p1_05_pin_unpin
```

## 👀 What to Observe

You will see the self-referential struct safely print its data via an internal pointer. The stream iteration will also work correctly because the stream has been pinned to the stack.

## 🔗 How This Connects

With an understanding of the mechanics of futures (Futures, Wakers, and Pinning), we can now move on to `06_executors_runtimes` to see how Tokio runs these futures.

## 🏋️ Your Turn

- Comment out `tokio::pin!(stream);` and try to run `stream.next().await`. Read the compiler error carefully.
