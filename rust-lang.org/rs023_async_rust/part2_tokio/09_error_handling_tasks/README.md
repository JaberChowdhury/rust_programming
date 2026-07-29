# 09_error_handling_tasks

## 🎯 Concept
Handle all task failure modes.

## 🧠 Key Ideas
- JoinError handling
- Panics in async tasks
- Supervisor pattern

## 📦 Dependencies
| Crate | Reason |
| --- | --- |
| tokio | runtime |
| anyhow | error handling |

## ▶️ How to Run
cargo run -p p2_09_error_handling_tasks

## 👀 What to Observe
Tasks failing being handled gracefully.

## 🔗 How This Connects
Robustness in production.

## 🏋️ Your Turn
1. Handle task cancellation.
