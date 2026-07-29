# 11_async_channels

## 🎯 Concept

This exercise demonstrates the four primary channel types provided by Tokio, each designed for a specific communication pattern between tasks.

## 🧠 Key Ideas

- `mpsc`: Work queues. Many senders, one receiver.
- `oneshot`: Request/Response. One sender, one receiver, single message.
- `broadcast`: Pub/Sub. Many senders, many receivers, all receivers get all messages.
- `watch`: State distribution. One sender, many receivers, receivers only care about the *latest* value.

## 📦 Dependencies

| Crate | Reason |
| --- | --- |
| tokio | For the async runtime and all channel types (`tokio::sync`). |

## ▶️ How to Run

```bash
cargo run -p p1_11_async_channels
```

## 👀 What to Observe

You will see messages successfully routed through all four channel types, demonstrating their specific behaviors (e.g., both broadcast subscribers receiving the same message).

## 🔗 How This Connects

While channels are the preferred way to share data ("share memory by communicating"), sometimes you really need shared state. `12_sync_primitives` covers async-safe locks.

## 🏋️ Your Turn

- Try modifying the `mpsc` buffer size to 1 and see if it affects execution speed (it might add backpressure).
- Create a `broadcast` channel, send a message, *then* subscribe. What happens when the subscriber tries to `recv()`?
