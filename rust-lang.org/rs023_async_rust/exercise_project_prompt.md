# 🦀 Exercise Project Generation Prompt

> Copy everything below this line and paste it to the model.

---

## SYSTEM CONTEXT

You are an expert Rust engineer and educator. Your task is to build a **comprehensive, hands-on exercise project** that teaches async Rust, Tokio, and API development in Rust. You will create a structured Cargo workspace with multiple sub-crates, each focused on a specific concept from the roadmap below. Every exercise must include:

1. **Working, runnable Rust code** (no pseudocode, no `todo!()` left unexplained)
2. **A detailed `README.md`** per folder explaining the concept, how to run it, and what to observe
3. **Inline code comments** explaining every non-obvious line
4. **A root-level `README.md`** that serves as an index and learning guide for the entire project

---

## TARGET DIRECTORY

All files go inside:

```
/home/jaber/Documents/code/rust_programming/rust-lang.org/rs023_async_rust/
```

---

## WORKSPACE STRUCTURE

Create the following **Cargo workspace** layout. Every sub-folder is a separate Cargo crate (either `[[bin]]` or `[lib]` as appropriate). Do NOT use a single flat `main.rs` for everything.

```
rs023_async_rust/
├── Cargo.toml                        ← Workspace root
├── README.md                         ← Master index + learning guide
├── rust_async_tokio_api_roadmap.md   ← Already exists, do not touch
│
├── part1_async_rust/
│   ├── README.md
│   ├── 01_why_async/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 02_threads_vs_async/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 03_future_trait/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 04_async_await_syntax/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 05_pin_unpin/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 06_executors_runtimes/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 07_async_error_handling/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 08_spawning_tasks/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 09_join_select/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 10_async_streams/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 11_async_channels/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 12_sync_primitives/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 13_timeouts_cancellation/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 14_joinset/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   └── 15_custom_future/
│       ├── Cargo.toml
│       ├── README.md
│       └── src/main.rs
│
├── part2_tokio/
│   ├── README.md
│   ├── 01_tokio_setup/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 02_tokio_runtime/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 03_tokio_io_traits/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 04_tcp_networking/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/
│   │       ├── main.rs      ← runs both server + client demo
│   │       ├── server.rs
│   │       └── client.rs
│   ├── 05_tokio_time/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 06_task_management/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 07_channels_deep_dive/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/
│   │       ├── main.rs
│   │       ├── mpsc_demo.rs
│   │       ├── broadcast_demo.rs
│   │       ├── watch_demo.rs
│   │       └── oneshot_demo.rs
│   ├── 08_tokio_streams/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 09_error_handling_tasks/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 10_graceful_shutdown/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 11_tracing_observability/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   ├── 12_testing_async/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/
│   │       ├── main.rs
│   │       └── lib.rs      ← contains #[tokio::test] examples
│   ├── 13_performance_tuning/
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src/main.rs
│   └── 14_tokio_console/
│       ├── Cargo.toml
│       ├── README.md
│       └── src/main.rs
│
└── part3_api/
    ├── README.md
    └── my_api/                       ← Single full Axum API project
        ├── Cargo.toml
        ├── README.md
        ├── .env.example
        ├── Dockerfile
        ├── config/
        │   └── default.toml
        ├── migrations/
        │   └── 001_init.sql
        └── src/
            ├── main.rs
            ├── config.rs
            ├── error.rs
            ├── state.rs
            ├── routes/
            │   ├── mod.rs
            │   ├── health.rs
            │   ├── users.rs
            │   └── auth.rs
            ├── models/
            │   ├── mod.rs
            │   └── user.rs
            ├── middleware/
            │   ├── mod.rs
            │   └── auth.rs
            └── db/
                ├── mod.rs
                └── users.rs
```

---

## WORKSPACE `Cargo.toml`

Create the root `Cargo.toml` as a workspace that includes all sub-crates:

```toml
[workspace]
resolver = "2"
members = [
    "part1_async_rust/01_why_async",
    "part1_async_rust/02_threads_vs_async",
    # ... all crates
]
```

---

## PER-EXERCISE REQUIREMENTS

### For EVERY exercise crate (`01_why_async` through `14_tokio_console`):

#### `Cargo.toml` must:

- Have a clear `[package]` name (e.g., `name = "p1_01_why_async"`)
- Include only the dependencies actually used in that exercise
- Use exact versions (no `*`)

#### `src/main.rs` must:

- Be **fully runnable** with `cargo run`
- Contain a `fn main()` or `#[tokio::main] async fn main()`
- Demonstrate the concept with **real, observable output** (use `println!`, `eprintln!`, timing measurements)
- Include `// CONCEPT:` comments at the top explaining what this file teaches
- Include `// WHY:` comments explaining non-obvious decisions
- Use `// EXERCISE:` comment blocks at the bottom with 2–3 suggested modifications the learner can try themselves

#### `README.md` must contain these exact sections:

```markdown
# [Exercise Name]

## 🎯 Concept

[2–3 sentence explanation of the core concept being taught]

## 🧠 Key Ideas

- Bullet list of the 3–5 most important things to understand

## 📦 Dependencies

[Table of crates used and why]

## ▶️ How to Run

[Exact cargo commands]

## 👀 What to Observe

[What output to expect and what it means]

## 🔗 How This Connects

[How this concept connects to the previous and next exercise]

## 🏋️ Your Turn

[2–3 concrete exercises for the learner to try]
```

---

## DETAILED EXERCISE SPECIFICATIONS

### PART 1 — ASYNC RUST

---

#### `01_why_async`

**Goal:** Show the difference between blocking and non-blocking I/O through simulation.

- Simulate 5 "network requests" each taking 200ms
- Version A: Run them sequentially (blocking — use `std::thread::sleep`)
- Version B: Run them concurrently (async — use `tokio::time::sleep`)
- Print timestamps showing total elapsed time for both approaches
- **Expected output**: Sequential ~1000ms, Async ~200ms

---

#### `02_threads_vs_async`

**Goal:** Demonstrate threads vs async tasks for concurrent work.

- Spawn 1000 OS threads vs 1000 async tasks, each sleeping 1s
- Measure memory usage (print `std::mem::size_of` for context) and wall clock time
- Show that async tasks are lighter — demonstrate with JoinHandle for both
- Include a `spawn_blocking` example for CPU work

---

#### `03_future_trait`

**Goal:** Implement `Future` manually — no async/await, no runtime magic.

- Create a `struct ReadyFuture<T>` that immediately returns `Poll::Ready(T)`
- Create a `struct PendingOnceFuture` that returns `Poll::Pending` on first poll, `Poll::Ready(())` on second
- Create a manual executor using `std::task::Wake` and `Arc<Mutex<VecDeque<Task>>>`
- Show the polling loop in detail with `println!` at each `poll()` call
- Run all futures through your custom executor (no Tokio here!)

---

#### `04_async_await_syntax`

**Goal:** Show how `async fn` desugars and how `.await` works.

- Write a function with `async fn` and show its desugared form as a comment
- Show chaining multiple `.await` calls
- Show `async {}` blocks
- Show returning different types: `()`, `String`, `Result<T, E>`
- Show that calling `async fn` without `.await` does nothing (lazy evaluation demo)

---

#### `05_pin_unpin`

**Goal:** Understand why `Pin` exists through a self-referential struct example.

- Create a self-referential struct that would be unsafe to move
- Show compilation errors when trying to move it without `Pin`
- Use `Box::pin()` to heap-allocate and pin a future
- Use `tokio::pin!()` macro for stack pinning
- Show a stream that requires pinning to iterate

---

#### `06_executors_runtimes`

**Goal:** Compare different runtime configurations.

- Show `current_thread` runtime (single thread)
- Show `multi_thread` runtime with 1, 2, and N workers
- Demonstrate how tasks are distributed across threads using `std::thread::current().id()`
- Show `Runtime::block_on` vs `#[tokio::main]`

---

#### `07_async_error_handling`

**Goal:** Master error propagation in async code.

- Show `?` operator in async functions
- Create custom error types with `thiserror`
- Show `anyhow::Result` for application code
- Show error wrapping and context with `.context()`
- Demonstrate error recovery with `unwrap_or_else`, `map_err`
- Show error in spawned tasks (double Result: `Result<Result<T, E>, JoinError>`)

---

#### `08_spawning_tasks`

**Goal:** Understand `tokio::spawn` and task lifecycle.

- Spawn 10 tasks that each do work and return a value
- Collect all `JoinHandle`s and await them
- Show that spawned tasks are `'static + Send`
- Show the error when trying to capture a non-`Send` type (use a comment with `// THIS WONT COMPILE:`)
- Use `Arc` to share data between tasks correctly
- Show task cancellation by dropping a `JoinHandle`

---

#### `09_join_select`

**Goal:** Master concurrent future execution.

- `tokio::join!`: Fetch 3 "APIs" concurrently, collect all results
- `tokio::try_join!`: Same but short-circuit on first error — simulate one failure
- `tokio::select!`: Race two futures, first one wins; demonstrate cancellation of loser
- `futures::future::join_all`: Dynamic number of futures in a Vec
- Show the difference in total elapsed time vs sequential execution

---

#### `10_async_streams`

**Goal:** Process a sequence of async values.

- Create a stream from a `Vec` using `tokio_stream::iter`
- Create a stream from an `mpsc` channel using `ReceiverStream`
- Create a custom stream using the `stream!` macro (async generator style)
- Use stream combinators: `.map()`, `.filter()`, `.take()`, `.collect()`
- Show `.for_each_concurrent()` for parallel stream processing
- Simulate a paginated API call that returns pages as a stream

---

#### `11_async_channels`

**Goal:** All four Tokio channel types with real use cases.

- **mpsc**: Work queue — producer spawns 10 tasks, consumer processes them
- **oneshot**: Request/response — ask actor for a value, get reply
- **broadcast**: Chat room simulation — 3 subscribers receive same messages
- **watch**: Config hot-reload simulation — workers observe latest config value
- Each demo clearly labeled with `println!` showing which channel is being demonstrated

---

#### `12_sync_primitives`

**Goal:** Async-safe shared state.

- `tokio::sync::Mutex`: Shared counter incremented by 100 concurrent tasks
- `tokio::sync::RwLock`: Cache with many readers and occasional writers
- `tokio::sync::Semaphore`: Limit concurrent HTTP connections to a pool of 5
- `tokio::sync::Barrier`: Coordinate N tasks to all start simultaneously
- Show the **deadlock** that happens with `std::sync::Mutex` across `.await` (as a comment — don't actually deadlock)

---

#### `13_timeouts_cancellation`

**Goal:** Handle slow operations gracefully.

- `tokio::time::timeout`: Wrap a slow future, handle the timeout case
- `select!` for cancellation: Cancel a long computation when a signal arrives
- Implement a retry-with-backoff pattern using `tokio::time::sleep`
- Show that dropped futures are automatically cancelled (cooperative cancellation)
- Build a "deadline propagation" helper function

---

#### `14_joinset`

**Goal:** Structured concurrency with `JoinSet`.

- Process a list of 20 URLs (simulated) concurrently with `JoinSet`
- Limit concurrency to 5 at a time using a `Semaphore`
- Collect results as they complete (not in submission order)
- Handle individual task failures without cancelling the whole set
- Show `JoinSet::abort_all()` for cleanup

---

#### `15_custom_future`

**Goal:** Build a real `Future` implementation from scratch.

- Implement `struct TimerFuture { deadline: Instant }` that uses a real thread-based waker
- Implement a `struct YieldNow` that yields once then completes (like `tokio::task::yield_now`)
- Implement `struct Poll3Times<F>` that wraps another future and logs every poll
- Show how `cx.waker().wake_by_ref()` triggers re-polling
- Run all custom futures with Tokio

---

### PART 2 — TOKIO

---

#### `01_tokio_setup`

**Goal:** Correct Tokio setup and feature flags.

- Show `[dependencies]` with individual features vs `full`
- Demo `current_thread`, `multi_thread` flavors
- Show `#[tokio::main]` macro expansion as a comment
- Show `tokio::runtime::Handle::current()` to access runtime from sync code

---

#### `02_tokio_runtime`

**Goal:** Runtime internals and configuration.

- Build a runtime manually with `Builder`
- Show `worker_threads`, `thread_name`, `on_thread_start` hooks
- Demonstrate `runtime.spawn()`, `runtime.block_on()`, `runtime.enter()`
- Show `tokio::runtime::Handle` for spawning from non-async contexts

---

#### `03_tokio_io_traits`

**Goal:** Async file I/O with Tokio.

- Read a file with `tokio::fs::read_to_string`
- Write a file with `tokio::fs::write`
- Use `BufReader`/`BufWriter` for large files
- Show `AsyncReadExt` and `AsyncWriteExt` extension methods
- Copy a file asynchronously using `tokio::io::copy`
- Create a temp file, write to it, read back — clean up after

---

#### `04_tcp_networking`

**Goal:** Build a working TCP echo server and client.

- `server.rs`: `TcpListener` that accepts connections, spawns per-connection task, echoes back
- `client.rs`: `TcpStream` that connects, sends 5 messages, reads responses
- `main.rs`: Spawns server task, runs client, shows full conversation
- Add a line protocol (newline-delimited messages) using `BufReader::lines()`
- Show connection count tracking with `Arc<AtomicUsize>`

---

#### `05_tokio_time`

**Goal:** All Tokio time utilities.

- `sleep`: Non-blocking delay — show it doesn't block other tasks
- `interval`: Tick every 500ms for 5 ticks, show missed tick behavior
- `timeout`: Wrap a 2s operation with a 1s timeout
- `Instant`: Measure async operation duration
- `interval_at`: Start interval at a specific time in the future
- Build a simple "heartbeat" task using `interval`

---

#### `06_task_management`

**Goal:** Master `spawn`, `spawn_blocking`, `spawn_local`, `JoinHandle`.

- Spawn 5 async tasks, await all `JoinHandle`s
- Use `spawn_blocking` for a CPU-heavy computation (sum of large range)
- Show timing: `spawn_blocking` doesn't block async workers
- Show task IDs and thread IDs to visualize execution
- Demonstrate `JoinHandle::abort()` for cancellation

---

#### `07_channels_deep_dive`

**Goal:** Actor model pattern using channels.

- Build a `CacheActor` with `mpsc` that handles `Get`/`Set`/`Delete` commands
- Use `oneshot` channels embedded in commands for replies
- Build a `PubSub` system with `broadcast`
- Build a `ConfigWatcher` with `watch` that notifies workers on change
- Show backpressure: what happens when `mpsc` buffer is full

---

#### `08_tokio_streams`

**Goal:** Async streams and `tokio-stream`.

- Convert `mpsc::Receiver` to stream with `ReceiverStream`
- Use `stream!` macro to build a custom async generator
- Chain stream adapters: `map`, `filter`, `chunks`, `throttle`
- Implement `merge` of two streams
- Show `StreamExt::timeout` to add per-item timeouts

---

#### `09_error_handling_tasks`

**Goal:** Handle all task failure modes.

- Show the double-`Result` from `JoinHandle<Result<T, E>>`
- Distinguish panic vs error vs cancellation from `JoinError`
- Use `JoinSet` and handle mixed success/failure results
- Show `catch_unwind` equivalent in async
- Build a supervisor pattern that restarts failed tasks

---

#### `10_graceful_shutdown`

**Goal:** Production-grade shutdown handling.

- Use `tokio::signal::ctrl_c()` to detect Ctrl+C
- Broadcast shutdown signal to all workers via `broadcast::channel`
- Use `select!` in each worker to check shutdown signal
- Add a `tokio::time::timeout` as a "force shutdown" deadline
- Show connection draining (finish current work before stopping)

---

#### `11_tracing_observability`

**Goal:** Structured logging and distributed tracing.

- Set up `tracing_subscriber` with `EnvFilter`
- Use all log levels: `trace!`, `debug!`, `info!`, `warn!`, `error!`
- Use `#[instrument]` on async functions
- Add custom fields to spans
- Show JSON output format for production
- Demonstrate span context propagation across tasks

---

#### `12_testing_async`

**Goal:** Write correct async tests.

- `#[tokio::test]` basic async test
- Test with `tokio::time::pause()` + `advance()` (no real waiting!)
- Mock a dependency using a trait + test implementation
- Test a channel-based actor
- Use `tokio::test(flavor = "multi_thread")` for tests requiring multiple threads

---

#### `13_performance_tuning`

**Goal:** Understand Tokio performance characteristics.

- Benchmark task spawning overhead (spawn 10k tasks, measure time)
- Show cost of `Mutex` contention with varying thread counts
- Demonstrate `spawn_blocking` offloading and its overhead
- Show benefit of `BufReader`/`BufWriter` with benchmarks
- Runtime tuning: `worker_threads`, `max_blocking_threads`
- Tips printed as `println!` with measured numbers

---

#### `14_tokio_console`

**Goal:** Set up `tokio-console` for runtime inspection.

- Add `console-subscriber` dependency
- Initialize it before the runtime
- Spawn several long-running tasks with descriptive names via `task::Builder`
- Show a task that is starved (holds CPU too long without yielding)
- Include `README.md` with screenshots/instructions for running `tokio-console` CLI

---

### PART 3 — API (`part3_api/my_api`)

Build a **complete, runnable REST API** with the following endpoints:

```
GET    /health
POST   /auth/register
POST   /auth/login
GET    /users          (protected, paginated)
GET    /users/:id      (protected)
PUT    /users/:id      (protected, owner only)
DELETE /users/:id      (protected, owner only)
```

#### Required files and their responsibilities:

| File                 | Responsibility                                                           |
| -------------------- | ------------------------------------------------------------------------ |
| `main.rs`            | Bootstrap: load config, init DB pool, build router, serve                |
| `config.rs`          | `Settings` struct, loaded from env + `config/default.toml`               |
| `error.rs`           | `ApiError` enum + `IntoResponse` impl                                    |
| `state.rs`           | `AppState` struct with db pool + config                                  |
| `routes/mod.rs`      | Combine all routers, apply middleware stack                              |
| `routes/health.rs`   | `GET /health` — returns uptime, version, db status                       |
| `routes/users.rs`    | All user CRUD endpoints                                                  |
| `routes/auth.rs`     | Register + Login, returns JWT                                            |
| `models/user.rs`     | `User`, `CreateUserRequest`, `UpdateUserRequest`, `UserResponse` structs |
| `middleware/auth.rs` | `AuthUser` extractor that validates JWT                                  |
| `db/users.rs`        | All database query functions (using `sqlx`)                              |

#### API Requirements:

- Passwords hashed with `bcrypt` or `argon2`
- JWT tokens with 24h expiry
- Pagination on `GET /users` (`?page=1&per_page=20`)
- Proper HTTP status codes: 200, 201, 400, 401, 403, 404, 422, 500
- JSON error responses: `{ "error": "...", "status": 422 }`
- CORS, request logging (tracing), response compression middleware
- `.env.example` with all required variables documented
- `migrations/001_init.sql` with correct schema
- `Dockerfile` with multi-stage build
- `README.md` with: setup instructions, env vars table, API endpoint table, curl examples for every endpoint, how to run migrations

---

## README STANDARDS

### Root `README.md` must include:

1. Project title and description
2. Prerequisites (Rust version, toolchain, Postgres for Part 3)
3. How to clone and build the workspace
4. A **table** listing all exercises with one-line descriptions
5. Learning path recommendation (which order to do them)
6. How to run any single exercise: `cargo run -p p1_01_why_async`
7. Links to official resources (Tokio docs, async book, Zero To Production)

### Part-level `README.md` (3 total) must include:

1. What this part covers
2. Prerequisites for this part
3. Ordered list of exercises with brief descriptions
4. Key concepts checklist (the learner can tick these off)

---

## CODE QUALITY STANDARDS

1. **No warnings**: All code must compile with `cargo build` without warnings
2. **No `unwrap()` in library code**: Use `?` or explicit error handling; `unwrap()` only in `main()` where it's clear why it's safe
3. **Consistent formatting**: All code must pass `cargo fmt`
4. **Clippy clean**: Code should not trigger `cargo clippy` warnings
5. **Real output**: Every `main.rs` produces visible, meaningful console output when run
6. **No placeholder functions**: No `fn todo_this() { todo!() }` — implement everything
7. **Comments over cleverness**: Prefer clear, documented code over terse one-liners

---

## DEPENDENCY VERSIONS TO USE

```toml
tokio = { version = "1.38", features = ["full"] }
futures = "0.3"
tokio-stream = "0.1"
async-stream = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Part 3 only
axum = { version = "0.7", features = ["macros"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "compression-gzip", "request-id"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "time", "migrate"] }
jsonwebtoken = "9"
bcrypt = "0.15"
uuid = { version = "1", features = ["v4", "serde"] }
config = "0.14"
dotenvy = "0.15"
validator = { version = "0.18", features = ["derive"] }
axum-valid = "0.19"
console-subscriber = "0.3"   # Part 2, exercise 14 only
```

---

## EXECUTION ORDER

Build the files in this exact order to avoid dependency issues:

1. Root `Cargo.toml` (workspace definition)
2. All Part 1 crates (no external runtime dependencies except `tokio`)
3. All Part 2 crates
4. Part 3 API (`my_api`) — most complex, build last
5. All `README.md` files (root, part-level, exercise-level)

---

## FINAL CHECKLIST

Before finishing, verify:

- [ ] `cargo build --workspace` succeeds with no errors
- [ ] Every crate has its own `Cargo.toml` and `src/main.rs`
- [ ] Root `Cargo.toml` lists all workspace members
- [ ] Every exercise folder has a `README.md`
- [ ] Part 3 API has all source files implemented
- [ ] All `README.md` files follow the specified template
- [ ] `.env.example` exists in `part3_api/my_api/`
- [ ] `Dockerfile` exists in `part3_api/my_api/`
- [ ] `migrations/001_init.sql` exists in `part3_api/my_api/`
