# 15_custom_future

## 🎯 Concept

This exercise brings everything together by showing how to manually implement robust `Future` state machines that interact correctly with `Waker`s.

## 🧠 Key Ideas

- If a Future returns `Poll::Pending`, it **must** arrange for `waker.wake()` to be called when it can make progress.
- If it fails to do so, it will hang forever (task starvation).
- `cx.waker().wake_by_ref()` allows a future to immediately re-queue itself (useful for cooperative yielding).
- Future wrappers (like `Poll3Times`) use `Pin::new` to project pinning guarantees to their inner futures.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | Async runtime to execute our custom futures. |

## ▶️ How to Run

```bash
cargo run -p p1_15_custom_future
```

## 👀 What to Observe

You will see the `TimerFuture` pause execution for 500ms using a background OS thread to trigger the waker. You will also see the `YieldNow` future return `Pending` once, and `Poll3Times` logging each time its inner future is polled.

## 🔗 How This Connects

This concludes Part 1 (Async Rust)! You now know both the low-level mechanics of Futures and the high-level macros and synchronization tools to manage them.

## 🏋️ Your Turn

- Create a `Future` that wraps `tokio::time::sleep` but prints a message every time it gets polled.
