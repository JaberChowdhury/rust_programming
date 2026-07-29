# 11_tracing_observability

## 🎯 Concept
Structured logging and distributed tracing.

## 🧠 Key Ideas
- tracing setup
- #[instrument]
- json output

## 📦 Dependencies
| Crate | Reason |
| --- | --- |
| tokio | runtime |
| tracing | macros |
| tracing-subscriber | log emission |

## ▶️ How to Run
RUST_LOG=debug cargo run -p p2_11_tracing_observability

## 👀 What to Observe
Structured logs with spans.

## 🔗 How This Connects
Debugging complex async applications.

## 🏋️ Your Turn
1. Output JSON formatted logs.
