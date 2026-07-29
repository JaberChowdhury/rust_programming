# 06_task_management

## 🎯 Concept
Master spawn, spawn_blocking, spawn_local, JoinHandle.

## 🧠 Key Ideas
- tokio::spawn
- spawn_blocking
- JoinHandle

## 📦 Dependencies
| Crate | Reason |
| --- | --- |
| tokio | Task management |

## ▶️ How to Run
cargo run -p p2_06_task_management

## 👀 What to Observe
Tasks running concurrently, blocking task offloaded.

## 🔗 How This Connects
Basis for concurrency in Tokio apps.

## 🏋️ Your Turn
1. Spawn 1000 tasks.
