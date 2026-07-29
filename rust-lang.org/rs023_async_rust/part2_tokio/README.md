# Part 2 — Tokio

This part covers the Tokio runtime, I/O traits, time, networking, task management, streams, testing, tracing, and console.

## Prerequisites
- Rust and Cargo installed
- Basic understanding of async Rust (from Part 1)

## Exercises
1. `01_tokio_setup`: Setup and features
2. `02_tokio_runtime`: Runtime internals
3. `03_tokio_io_traits`: Async I/O
4. `04_tcp_networking`: TCP server/client
5. `05_tokio_time`: Time utilities
6. `06_task_management`: Spawn, spawn_blocking, JoinHandle
7. `07_channels_deep_dive`: Actor pattern and channels
8. `08_tokio_streams`: Streams and tokio-stream
9. `09_error_handling_tasks`: Handle failure modes
10. `10_graceful_shutdown`: Production shutdown
11. `11_tracing_observability`: Tracing
12. `12_testing_async`: Testing async code
13. `13_performance_tuning`: Performance characteristics
14. `14_tokio_console`: Console subscriber

## Key Concepts Checklist
- [ ] Tokio setup and runtime flavors
- [ ] Async file I/O
- [ ] TCP Networking
- [ ] Time and timeouts
- [ ] Task management and spawned blocking
- [ ] Channels (mpsc, broadcast, watch, oneshot)
- [ ] Stream adapters
- [ ] Graceful shutdown and select!
- [ ] Tracing subscriber
- [ ] Tokio testing macros
- [ ] Tokio console for inspection
