# 10_graceful_shutdown

## 🎯 Concept
Production-grade shutdown handling.

## 🧠 Key Ideas
- tokio::signal::ctrl_c
- broadcast channel for signal
- select! for shutdown checking

## 📦 Dependencies
| Crate | Reason |
| --- | --- |
| tokio | runtime and signals |

## ▶️ How to Run
cargo run -p p2_10_graceful_shutdown

## 👀 What to Observe
Ctrl+C gracefully shutting down tasks.

## 🔗 How This Connects
Proper lifecycle management of services.

## 🏋️ Your Turn
1. Add force timeout for shutdown.
