# 🦀 Async Rust, Tokio & API Development

Welcome to the **Async Rust, Tokio & API Development** hands-on exercise project! This workspace contains a comprehensive set of exercises designed to take you from the fundamentals of asynchronous Rust all the way to building production-ready REST APIs with Axum and Tokio.

## Prerequisites

Before you begin, ensure you have the following installed:

*   **Rust Toolchain**: `rustup` installed with at least Rust 1.79.0. (Install via [rustup.rs](https://rustup.rs/))
*   **Docker & Docker Compose**: Helpful for running the Postgres database in Part 3.
*   **PostgreSQL**: (Part 3) You need a running Postgres database instance to build and test the API.

## How to Clone and Build

To get started, clone this repository and build the workspace:

```bash
git clone <your-repo-url>
cd rs023_async_rust
cargo build --workspace
```

## How to Run an Exercise

Every exercise folder is a separate Cargo crate. You can run any specific exercise from the root directory using the `-p` (package) flag:

```bash
cargo run -p p1_01_why_async
```

For Part 3 (the API), you would first set up your `.env` file and database, then run:

```bash
cargo run -p my_api
```

## Exercises Overview

This workspace is divided into three major parts.

### Part 1: Async Rust Foundations

| Crate Name | Description |
| :--- | :--- |
| `p1_01_why_async` | Simulates blocking vs. non-blocking I/O to demonstrate why async is useful. |
| `p1_02_threads_vs_async` | Compares the overhead of OS threads against async tasks. |
| `p1_03_future_trait` | Manually implements the `Future` trait without `async`/`await`. |
| `p1_04_async_await_syntax` | Explores how `async fn` desugars and how `.await` works under the hood. |
| `p1_05_pin_unpin` | Demonstrates the need for `Pin` with self-referential structs. |
| `p1_06_executors_runtimes` | Configures single-threaded vs. multi-threaded runtimes. |
| `p1_07_async_error_handling` | Covers error propagation and handling in async functions. |
| `p1_08_spawning_tasks` | Uses `tokio::spawn` and manages task lifecycles with `JoinHandle`. |
| `p1_09_join_select` | Master concurrent execution using `tokio::join!` and `tokio::select!`. |
| `p1_10_async_streams` | Processes sequences of async values using streams. |
| `p1_11_async_channels` | Deep dive into `mpsc`, `oneshot`, `broadcast`, and `watch` channels. |
| `p1_12_sync_primitives` | Safe shared state using async-aware Mutexes and RwLocks. |
| `p1_13_timeouts_cancellation` | Handling slow operations with timeouts and cancellation. |
| `p1_14_joinset` | Structured concurrency for managing multiple tasks with `JoinSet`. |
| `p1_15_custom_future` | Building a real `Future` implementation and waker from scratch. |

### Part 2: Mastering Tokio

| Crate Name | Description |
| :--- | :--- |
| `p2_01_tokio_setup` | Tokio feature flags and accessing the runtime. |
| `p2_02_tokio_runtime` | Runtime internals, custom configuration, and `spawn_blocking`. |
| `p2_03_tokio_io_traits` | Async file I/O with `AsyncRead` and `AsyncWrite`. |
| `p2_04_tcp_networking` | Building a working TCP echo server and client. |
| `p2_05_tokio_time` | Tokio time utilities (`sleep`, `interval`, `timeout`). |
| `p2_06_task_management` | CPU-heavy task offloading with `spawn_blocking`. |
| `p2_07_channels_deep_dive` | Building the Actor model using Tokio channels. |
| `p2_08_tokio_streams` | Advanced stream processing and combinators. |
| `p2_09_error_handling_tasks` | Handling task failure modes and panics. |
| `p2_10_graceful_shutdown` | Implementing production-grade graceful shutdown handling. |
| `p2_11_tracing_observability` | Structured logging and distributed tracing with `tracing`. |
| `p2_12_testing_async` | Writing tests with time control using `#[tokio::test]`. |
| `p2_13_performance_tuning` | Understanding Tokio performance characteristics and benchmarking. |
| `p2_14_tokio_console` | Setting up `tokio-console` for runtime inspection and debugging. |

### Part 3: API Development

| Crate Name | Description |
| :--- | :--- |
| `my_api` | A complete, production-ready REST API built with Axum, Tokio, and SQLx. |

## Learning Path Recommendation

For the best experience, progress linearly through the exercises:
1. Complete all of **Part 1** to solidify your understanding of futures, async/await, and basic concurrency.
2. Move on to **Part 2** to learn the specifics of the Tokio runtime, networking, and observability.
3. Finally, tackle **Part 3** to build a comprehensive real-world REST API integrating everything you've learned.

## Official Resources

*   [Tokio Documentation](https://tokio.rs/tokio/tutorial)
*   [Asynchronous Programming in Rust (Async Book)](https://rust-lang.github.io/async-book/)
*   [Zero To Production In Rust](https://www.zero2prod.com/)
