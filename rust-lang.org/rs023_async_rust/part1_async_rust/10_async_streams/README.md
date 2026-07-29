# 10_async_streams

## 🎯 Concept

This exercise introduces Async Streams, which are essentially async Iterators. A stream yields values over time, rather than all at once.

## 🧠 Key Ideas

- The `Stream` trait is the async version of `Iterator`.
- You retrieve values using `.next().await`.
- You can apply combinators like `map`, `filter`, and `take` just like standard iterators.
- The `async_stream` crate provides a handy `stream!` macro to write generator-like code using `yield`.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | Async runtime. |
| tokio-stream | Core stream traits and combinators (`StreamExt`). |
| async-stream | The `stream!` macro. |

## ▶️ How to Run

```bash
cargo run -p p1_10_async_streams
```

## 👀 What to Observe

You will see the values yielded sequentially. The custom stream simulates a paginated API, downloading a chunk, yielding elements individually, and repeating until exhausted.

## 🔗 How This Connects

Streams often produce values that are sent to other parts of an application. In `11_async_channels`, we'll explore how to send messages between independent tasks.

## 🏋️ Your Turn

- Modify the `fetch_page` to sleep for 500ms and notice the delayed yielding of the stream elements.
- Try collecting the `mapped_stream` into a `Vec` using `.collect::<Vec<_>>().await`.
