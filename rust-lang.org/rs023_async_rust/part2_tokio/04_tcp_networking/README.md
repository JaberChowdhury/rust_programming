# 04_tcp_networking

## 🎯 Concept
Build a working TCP echo server and client.

## 🧠 Key Ideas
- TcpListener
- TcpStream
- BufReader::lines

## 📦 Dependencies
| Crate | Reason |
| --- | --- |
| tokio | Async runtime and Networking |

## ▶️ How to Run
cargo run -p p2_04_tcp_networking

## 👀 What to Observe
Server accepts connection and echoes lines back to client.

## 🔗 How This Connects
Extends async I/O to networking.

## 🏋️ Your Turn
1. Allow server to handle multiple clients concurrently.
