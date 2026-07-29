# 14_joinset

## 🎯 Concept

This exercise introduces `JoinSet`, Tokio's tool for structured concurrency. It allows you to manage a dynamic collection of spawned tasks, reaping them as they complete rather than waiting for them in order.

## 🧠 Key Ideas

- Storing `JoinHandle`s in a `Vec` is fine for small numbers, but `JoinSet` is better for managing many tasks.
- `JoinSet::join_next()` yields the result of whichever task finishes *first*, regardless of insertion order.
- `JoinSet::abort_all()` instantly cancels every task currently running in the set.
- Using a `Semaphore` alongside a `JoinSet` is the standard pattern for limiting concurrency (e.g., "scrape 10,000 URLs, but only 50 at a time").

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | Async runtime, `JoinSet`, and `Semaphore`. |

## ▶️ How to Run

```bash
cargo run -p p1_14_joinset
```

## 👀 What to Observe

You will see 20 tasks processed with a maximum concurrency of 5. As tasks finish, they are logged immediately. The abort demo will show how quickly 10 hanging tasks can be killed and cleaned up.

## 🔗 How This Connects

We've covered almost all daily-use Tokio tools. In the final exercise for this section, `15_custom_future`, we will return to the `Future` trait to implement complex wrappers that integrate directly with the Waker system.

## 🏋️ Your Turn

- Alter the code to short-circuit (abort all and return) if *any* task returns a logic error.
