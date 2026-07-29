# 12_testing_async

## 🎯 Concept
Write correct async tests.

## 🧠 Key Ideas
- #[tokio::test]
- time pausing

## 📦 Dependencies
| Crate | Reason |
| --- | --- |
| tokio | runtime, test utils |

## ▶️ How to Run
cargo test -p p2_12_testing_async

## 👀 What to Observe
Async tests passing instantly despite sleeps.

## 🔗 How This Connects
Testing async business logic correctly.

## 🏋️ Your Turn
1. Test a channel actor.
