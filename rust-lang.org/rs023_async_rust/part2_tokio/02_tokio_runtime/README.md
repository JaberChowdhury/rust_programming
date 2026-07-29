# 02_tokio_runtime

## 🎯 Concept
Runtime internals and configuration.

## 🧠 Key Ideas
- Runtime Builder
- worker_threads, thread_name
- block_on

## 📦 Dependencies
| Crate | Reason |
| --- | --- |
| tokio | Async runtime |

## ▶️ How to Run
cargo run -p p2_02_tokio_runtime

## 👀 What to Observe
Threads being started and running async code.

## 🔗 How This Connects
Moving from macros to manual runtime control.

## 🏋️ Your Turn
1. Change worker thread count.
