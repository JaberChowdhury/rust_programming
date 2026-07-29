# 14_tokio_console

## 🎯 Concept
Set up tokio-console for runtime inspection.

## 🧠 Key Ideas
- console-subscriber
- Task naming

## 📦 Dependencies
| Crate | Reason |
| --- | --- |
| tokio | runtime |
| console-subscriber | telemetry |

## ▶️ How to Run
TOKIO_CONSOLE_BIND=127.0.0.1:6669 cargo run -p p2_14_tokio_console

## 👀 What to Observe
Open `tokio-console` in another terminal to see tasks.

## 🔗 How This Connects
Advanced debugging tool for async systems.

## 🏋️ Your Turn
1. Make a starved task and observe it in console.
