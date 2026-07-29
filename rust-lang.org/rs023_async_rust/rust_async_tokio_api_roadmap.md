# 🦀 Rust Async, Tokio & API Development — Complete Roadmap

> A structured, opinionated guide from beginner to production-ready.

---

## Table of Contents

- [Part 1 — Mastering Async Rust](#part-1--mastering-async-rust)
- [Part 2 — Mastering Tokio](#part-2--mastering-tokio)
- [Part 3 — API Development in Rust](#part-3--api-development-in-rust)

---

# Part 1 — Mastering Async Rust

> **Prerequisites**: Solid understanding of Rust fundamentals — ownership, borrowing, lifetimes, traits, generics, and error handling.

---

## 🟢 Phase 1 — Foundations of Concurrency (Week 1–2)

### Step 1: Understand Why Async Exists

Before writing a single line of async code, understand the **problem** it solves.

| Concept                | Description                                             |
| ---------------------- | ------------------------------------------------------- |
| **Blocking I/O**       | Thread sleeps while waiting for disk/network            |
| **Thread-per-request** | Expensive; OS threads have ~2MB stack each              |
| **Async I/O**          | Thread does other work while waiting; far more scalable |
| **Event Loop**         | Single-threaded loop that polls futures for completion  |

**Key Resources:**

- Read: [The Rust Book — Chapter 16 (Concurrency)](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- Read: [Asynchronous Programming in Rust (async book)](https://rust-lang.github.io/async-book/)

---

### Step 2: Understand Threads vs Async

```rust
// Threads — OS-managed, blocking
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Running in a thread");
    });
    handle.join().unwrap();
}
```

```rust
// Async — cooperatively scheduled, non-blocking
#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("{result}");
}
```

- **Use threads** for CPU-bound work (parsing, compression, crypto)
- **Use async** for I/O-bound work (HTTP, file I/O, database queries)

---

### Step 3: Learn the `Future` Trait

The `Future` trait is the bedrock of all async Rust. Everything async compiles down to a state machine implementing this trait.

```rust
use std::task::{Context, Poll};
use std::pin::Pin;

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

**What happens when you `.await` a Future:**

1. The executor calls `poll()` on the future
2. If the result is ready → `Poll::Ready(value)` is returned
3. If not ready → `Poll::Pending` is returned and a **Waker** is registered
4. When the I/O completes, the Waker wakes the executor to poll again

**Exercise:** Implement a simple `Future` manually without any runtime.

---

### Step 4: The `async`/`await` Syntax

```rust
// `async fn` returns an `impl Future<Output = T>`
async fn fetch_user(id: u32) -> String {
    // `.await` suspends this future until the inner future resolves
    let raw = http_get(&format!("/users/{id}")).await;
    raw
}

// async blocks
async fn run() {
    let user = async {
        fetch_user(42).await
    }.await;
}
```

**Key mental model:**

- `async fn foo()` → desugars to `fn foo() -> impl Future<Output = ...>`
- `.await` → polls the future, yields if `Poll::Pending`, resumes when ready

---

### Step 5: Understand `Pin` and `Unpin`

`Pin<P>` prevents a type from being moved in memory. This is crucial because async state machines hold references into themselves (self-referential structs).

```rust
use std::pin::Pin;

// Most types are Unpin — they can be freely moved
// Self-referential async state machines are !Unpin
```

**Rules to remember:**

- If a future is `Unpin`, you can use it normally
- If a future is `!Unpin`, you must `Box::pin(future)` or use `pin!()` macro
- You'll encounter this most with `tokio::pin!()` for streams

---

## 🟡 Phase 2 — Async Primitives (Week 3–4)

### Step 6: Executors and Runtimes

An **executor** is what actually drives futures to completion. Rust's `std` provides no executor — you must bring your own.

| Runtime       | Use Case                                    |
| ------------- | ------------------------------------------- |
| **Tokio**     | Production, network services, full-featured |
| **async-std** | Similar API to `std`, beginner-friendly     |
| **smol**      | Minimal, embeddable                         |
| **embassy**   | Embedded / no-std environments              |

For this roadmap, we focus on **Tokio** (most widely adopted).

---

### Step 7: `async` Error Handling

```rust
use std::error::Error;

// Returning Results from async fns
async fn read_file(path: &str) -> Result<String, std::io::Error> {
    let content = tokio::fs::read_to_string(path).await?;  // ? works!
    Ok(content)
}

// Using anyhow for ergonomic errors
async fn run() -> anyhow::Result<()> {
    let data = read_file("config.toml").await?;
    println!("{data}");
    Ok(())
}
```

**Libraries:**

- `anyhow` — flexible error propagation (applications)
- `thiserror` — derive-based custom error types (libraries)

---

### Step 8: Spawning Tasks

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    // Spawn a concurrent task — runs independently
    let handle = task::spawn(async {
        expensive_computation().await
    });

    // Do other work concurrently...
    do_something_else().await;

    // Wait for the spawned task to finish
    let result = handle.await.unwrap();
}
```

> [!IMPORTANT]
> Spawned tasks must be `'static + Send`. They cannot borrow from the calling scope.

---

### Step 9: Running Futures Concurrently

```rust
use tokio::join;
use futures::future;

// join! — run multiple futures concurrently, wait for ALL
async fn fetch_all() {
    let (users, posts, comments) = tokio::join!(
        fetch_users(),
        fetch_posts(),
        fetch_comments(),
    );
}

// select! — race futures, return FIRST to complete
async fn race() {
    tokio::select! {
        val = fetch_fast() => println!("fast: {val}"),
        val = fetch_slow() => println!("slow: {val}"),
    }
}

// try_join! — like join! but short-circuits on first Err
async fn fetch_all_or_fail() -> anyhow::Result<()> {
    let (a, b) = tokio::try_join!(
        fetch_a(),
        fetch_b(),
    )?;
    Ok(())
}
```

---

### Step 10: Async Streams

Streams are the async equivalent of iterators — a sequence of values produced over time.

```rust
use futures::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

async fn process_stream() {
    let mut stream = get_data_stream();

    // Process items as they arrive
    while let Some(item) = stream.next().await {
        process(item).await;
    }

    // Stream combinators
    stream
        .filter(|x| future::ready(*x > 0))
        .map(|x| x * 2)
        .for_each(|x| async move { println!("{x}") })
        .await;
}
```

---

## 🔴 Phase 3 — Advanced Async Patterns (Week 5–6)

### Step 11: Async Channels

```rust
use tokio::sync::{mpsc, oneshot, broadcast, watch};

// mpsc — multiple producer, single consumer (most common)
async fn mpsc_example() {
    let (tx, mut rx) = mpsc::channel::<String>(32);

    tokio::spawn(async move {
        tx.send("hello".to_string()).await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("Received: {msg}");
    }
}

// oneshot — send a single value once (great for request/response)
async fn oneshot_example() {
    let (tx, rx) = oneshot::channel::<u32>();
    tx.send(42).unwrap();
    let val = rx.await.unwrap();
}

// broadcast — multiple producers AND consumers (fan-out)
// watch — single producer, multiple consumers (latest value wins)
```

---

### Step 12: Async Synchronization Primitives

```rust
use tokio::sync::{Mutex, RwLock, Semaphore};
use std::sync::Arc;

// Async Mutex — like std::sync::Mutex but async-aware
async fn shared_state() {
    let data = Arc::new(Mutex::new(Vec::<u32>::new()));

    let data_clone = Arc::clone(&data);
    tokio::spawn(async move {
        let mut guard = data_clone.lock().await;  // async lock!
        guard.push(42);
    });
}

// Semaphore — limit concurrent access (e.g., connection pool)
async fn rate_limit() {
    let sem = Arc::new(Semaphore::new(10)); // max 10 concurrent

    let permit = sem.acquire().await.unwrap();
    make_http_request().await;
    drop(permit); // release
}
```

> [!WARNING]
> Never hold a `tokio::sync::Mutex` guard across an `.await` point if you're using `std::sync::Mutex`. Use `tokio::sync::Mutex` instead to prevent deadlocks.

---

### Step 13: Timeouts and Cancellation

```rust
use tokio::time::{timeout, Duration, sleep, interval};

async fn with_timeout() -> anyhow::Result<()> {
    // Cancel future if it takes longer than 5 seconds
    match timeout(Duration::from_secs(5), long_operation()).await {
        Ok(result) => println!("Done: {result:?}"),
        Err(_) => println!("Timed out!"),
    }
    Ok(())
}

// Graceful shutdown with select!
async fn graceful_shutdown(mut shutdown: tokio::sync::oneshot::Receiver<()>) {
    loop {
        tokio::select! {
            _ = do_work() => {},
            _ = &mut shutdown => {
                println!("Shutting down gracefully");
                break;
            }
        }
    }
}
```

---

### Step 14: Structured Concurrency with `JoinSet`

```rust
use tokio::task::JoinSet;

async fn process_many(ids: Vec<u32>) {
    let mut set = JoinSet::new();

    for id in ids {
        set.spawn(async move { fetch_user(id).await });
    }

    // Collect results as they complete (in any order)
    while let Some(result) = set.join_next().await {
        match result {
            Ok(user) => println!("Got: {user:?}"),
            Err(e) => eprintln!("Task panicked: {e}"),
        }
    }
}
```

---

### Step 15: Writing Your Own Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            // Register a waker — in real code, use a timer wheel
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn use_delay() {
    let delay = Delay { when: Instant::now() + Duration::from_secs(1) };
    delay.await;
    println!("Elapsed!");
}
```

---

## ✅ Phase 1–3 Checkpoint

Before moving on, you should be able to:

- [ ] Explain what a `Future` is and how polling works
- [ ] Write async functions and use `.await` correctly
- [ ] Spawn tasks with `tokio::spawn`
- [ ] Use `join!`, `select!`, and `try_join!`
- [ ] Work with async channels (`mpsc`, `oneshot`)
- [ ] Handle timeouts and cancellation
- [ ] Explain why `Pin` exists

---

# Part 2 — Mastering Tokio

> **Prerequisites**: Part 1 completed. Comfortable with async/await, futures, and basic concurrency.

---

## 🟢 Phase 1 — Tokio Fundamentals (Week 1–2)

### Step 1: Setting Up Tokio

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

**Feature flags (use only what you need in production):**

| Feature           | Description                          |
| ----------------- | ------------------------------------ |
| `rt`              | The async runtime itself             |
| `rt-multi-thread` | Multi-threaded scheduler             |
| `macros`          | `#[tokio::main]`, `#[tokio::test]`   |
| `net`             | TCP, UDP, Unix sockets               |
| `io-util`         | `AsyncRead`, `AsyncWrite` extensions |
| `fs`              | Async filesystem                     |
| `time`            | `sleep`, `timeout`, `interval`       |
| `sync`            | `Mutex`, `RwLock`, channels          |
| `signal`          | OS signal handling                   |
| `full`            | All features (dev/learning only)     |

---

### Step 2: The Tokio Runtime

```rust
// Single-threaded runtime (good for embedded/wasm)
#[tokio::main(flavor = "current_thread")]
async fn main() {}

// Multi-threaded runtime (default — best for servers)
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {}

// Manual runtime construction
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("my-worker")
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        // Your async entrypoint
    });
}
```

---

### Step 3: Tokio I/O Traits

Tokio provides async versions of `std::io` traits:

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

async fn read_write_example() -> anyhow::Result<()> {
    let file = tokio::fs::File::open("data.txt").await?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents).await?;

    let mut writer = BufWriter::new(tokio::fs::File::create("out.txt").await?);
    writer.write_all(contents.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}
```

---

### Step 4: TCP Networking

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Echo server
async fn run_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening on :8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Connection from {addr}");

        // Spawn a task for each connection
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buf = [0u8; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => break,  // Connection closed
            Ok(n) => {
                socket.write_all(&buf[..n]).await.unwrap();
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        }
    }
}
```

---

### Step 5: Tokio Time

```rust
use tokio::time::{sleep, interval, timeout, Duration, Instant};

async fn time_examples() {
    // Sleep (non-blocking — yields to executor)
    sleep(Duration::from_millis(100)).await;

    // Interval — tick at fixed rate
    let mut ticker = interval(Duration::from_secs(1));
    for _ in 0..5 {
        ticker.tick().await;
        println!("tick");
    }

    // Timeout
    let result = timeout(Duration::from_secs(3), slow_operation()).await;

    // Measure elapsed time
    let start = Instant::now();
    do_work().await;
    println!("Elapsed: {:?}", start.elapsed());
}
```

---

## 🟡 Phase 2 — Tokio in Depth (Week 3–4)

### Step 6: Task Management

```rust
use tokio::task::{spawn, spawn_blocking, spawn_local, JoinHandle, JoinSet};

async fn task_patterns() {
    // spawn — async task on the thread pool
    let h1: JoinHandle<u32> = spawn(async { compute_async().await });

    // spawn_blocking — run CPU-bound or blocking code off the async thread
    let h2: JoinHandle<u32> = spawn_blocking(|| {
        // This runs on a dedicated blocking thread pool
        (0..1_000_000u32).sum()
    });

    let (r1, r2) = tokio::join!(h1, h2);
    println!("{} {}", r1.unwrap(), r2.unwrap());
}
```

> [!TIP]
> Use `spawn_blocking` for: file system (std), database (sync drivers), CPU-heavy work, any `std::sync::Mutex` locks held for long periods.

---

### Step 7: Tokio Channels Deep Dive

```rust
use tokio::sync::{mpsc, broadcast, watch, oneshot};

// Pattern: actor model with mpsc
enum Command {
    Get { key: String, reply: oneshot::Sender<Option<String>> },
    Set { key: String, value: String },
}

async fn cache_actor(mut rx: mpsc::Receiver<Command>) {
    let mut store = std::collections::HashMap::new();

    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::Get { key, reply } => {
                let _ = reply.send(store.get(&key).cloned());
            }
            Command::Set { key, value } => {
                store.insert(key, value);
            }
        }
    }
}
```

---

### Step 8: Tokio Streams

```rust
use tokio_stream::{Stream, StreamExt};
use tokio_stream::wrappers::{ReceiverStream, BroadcastStream};

async fn stream_processing() {
    let (tx, rx) = tokio::sync::mpsc::channel(32);
    let mut stream = ReceiverStream::new(rx);

    // Producer
    tokio::spawn(async move {
        for i in 0..10u32 {
            tx.send(i).await.unwrap();
        }
    });

    // Consumer with combinators
    while let Some(item) = stream.next().await {
        println!("{item}");
    }
}

// Creating a custom stream
use tokio_stream::stream;

async fn number_stream() {
    let s = stream! {
        for i in 0..5 {
            tokio::time::sleep(Duration::from_millis(100)).await;
            yield i;
        }
    };
    tokio::pin!(s);
    while let Some(v) = s.next().await { println!("{v}"); }
}
```

---

### Step 9: Error Handling in Tokio Tasks

```rust
use tokio::task::JoinError;

async fn robust_task_handling() {
    let handle = tokio::spawn(async {
        risky_operation().await
    });

    match handle.await {
        Ok(Ok(result)) => println!("Success: {result}"),
        Ok(Err(e)) => eprintln!("Task error: {e}"),
        Err(join_err) if join_err.is_panic() => {
            eprintln!("Task panicked!");
        }
        Err(join_err) if join_err.is_cancelled() => {
            eprintln!("Task was cancelled!");
        }
        Err(e) => eprintln!("Unknown join error: {e}"),
    }
}
```

---

### Step 10: Graceful Shutdown

```rust
use tokio::signal;
use tokio::sync::broadcast;

async fn run_with_graceful_shutdown() -> anyhow::Result<()> {
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    // Spawn workers
    for i in 0..4 {
        let mut shutdown_rx = shutdown_tx.subscribe();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = do_work() => {},
                    _ = shutdown_rx.recv() => {
                        println!("Worker {i} shutting down");
                        break;
                    }
                }
            }
        });
    }

    // Wait for Ctrl+C
    signal::ctrl_c().await?;
    println!("Received shutdown signal");
    shutdown_tx.send(()).unwrap();

    // Give workers time to finish
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Shutdown complete");
    Ok(())
}
```

---

## 🔴 Phase 3 — Advanced Tokio (Week 5–6)

### Step 11: Tracing and Observability

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-opentelemetry = "0.22"  # Optional: OTEL integration
```

```rust
use tracing::{info, warn, error, debug, instrument, span, Level};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Initialize subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    process_request("req-123").await;
}

// Automatically instrument async functions
#[instrument(fields(request_id = %id))]
async fn process_request(id: &str) {
    info!("Processing request");
    let result = fetch_data().await;
    debug!(?result, "Fetch complete");
}
```

---

### Step 12: Testing Async Code

```rust
// Use #[tokio::test] for async tests
#[cfg(test)]
mod tests {
    use tokio::time::{pause, advance, Duration};

    #[tokio::test]
    async fn test_fetch_user() {
        let user = fetch_user(1).await.unwrap();
        assert_eq!(user.name, "Alice");
    }

    // Test timeouts without actually waiting
    #[tokio::test]
    async fn test_with_time_control() {
        tokio::time::pause(); // Freeze time

        let handle = tokio::spawn(async {
            sleep(Duration::from_secs(100)).await;
            "done"
        });

        tokio::time::advance(Duration::from_secs(101)).await;
        assert_eq!(handle.await.unwrap(), "done");
    }
}
```

---

### Step 13: Performance Tuning

```rust
// Tune the runtime for your workload
let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(num_cpus::get())           // Match CPU cores
    .max_blocking_threads(512)                  // For spawn_blocking pool
    .thread_stack_size(2 * 1024 * 1024)        // 2MB stack
    .on_thread_start(|| println!("Worker started"))
    .build()?;
```

**Performance tips:**

- Use `tokio-console` for runtime visualization and debugging
- Avoid holding locks (`Mutex`) across `.await` points
- Batch small I/O operations (use `BufReader`/`BufWriter`)
- Use `Arc<T>` instead of cloning large data
- Prefer channels over `Mutex<Vec<T>>` for inter-task communication
- Keep async tasks small; offload blocking work via `spawn_blocking`

---

### Step 14: `tokio-console` — Runtime Debugging

```toml
[dependencies]
console-subscriber = "0.2"
```

```rust
fn main() {
    console_subscriber::init(); // Must be first!

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main());
}
```

Run `tokio-console` in a separate terminal to visualize running tasks, identify slow futures, and detect starvation.

---

## ✅ Tokio Checkpoint

Before moving on, you should be able to:

- [ ] Configure Tokio runtime (single vs multi-thread)
- [ ] Use all four channel types appropriately
- [ ] Write a TCP server that handles multiple clients
- [ ] Implement graceful shutdown with signal handling
- [ ] Instrument async code with `tracing`
- [ ] Write `#[tokio::test]` tests including time-controlled tests
- [ ] Understand when to use `spawn_blocking`

---

# Part 3 — API Development in Rust

> **Prerequisites**: Parts 1 & 2 completed. Understand HTTP basics (verbs, status codes, headers, JSON).

---

## 🟢 Phase 1 — Choose Your Framework (Week 1)

### Step 1: Framework Comparison

| Framework     | Style                  | Best For                     |
| ------------- | ---------------------- | ---------------------------- |
| **Axum**      | Modular, tower-based   | Most projects — recommended  |
| **Actix-Web** | Actor model, high perf | Maximum throughput           |
| **Warp**      | Filter composition     | Functional style enthusiasts |
| **Poem**      | Full-featured          | OpenAPI-first development    |
| **Loco**      | Rails-like             | Full-stack Rust apps         |

> [!NOTE]
> This roadmap uses **Axum** — it integrates natively with Tokio, has excellent community support, and is maintained by the Tokio team.

---

### Step 2: Project Setup

```bash
cargo new my-api
cd my-api
```

```toml
# Cargo.toml
[dependencies]
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "compression-gzip"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1"
thiserror = "1"
```

---

### Step 3: Your First Axum API

```rust
use axum::{Router, Json, extract::Path};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/users/:id", axum::routing::get(get_user))
        .route("/users", axum::routing::post(create_user));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str { "OK" }

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User { id, name: "Alice".into(), email: "alice@example.com".into() })
}

async fn create_user(Json(payload): Json<User>) -> Json<User> {
    Json(payload)
}
```

---

## 🟡 Phase 2 — Core API Patterns (Week 2–3)

### Step 4: Proper Error Handling

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

// Define domain errors
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

// Implement IntoResponse to convert errors to HTTP responses
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(msg)    => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::Validation(msg)  => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            ApiError::Database(_)      => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()),
            ApiError::Internal(_)      => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
        };

        let body = Json(json!({ "error": message, "status": status.as_u16() }));
        (status, body).into_response()
    }
}

// Handlers return Result<T, ApiError>
async fn get_user(Path(id): Path<u32>) -> Result<Json<User>, ApiError> {
    let user = db::find_user(id)
        .await
        .map_err(|e| ApiError::Database(e))?
        .ok_or_else(|| ApiError::NotFound(format!("User {id} not found")))?;

    Ok(Json(user))
}
```

---

### Step 5: Request Validation

```toml
[dependencies]
validator = { version = "0.18", features = ["derive"] }
axum-valid = "0.19"
```

```rust
use validator::Validate;
use axum_valid::Valid;

#[derive(Deserialize, Validate)]
struct CreateUserRequest {
    #[validate(length(min = 2, max = 100))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 0, max = 150))]
    age: u8,
}

// axum-valid auto-validates and returns 422 on failure
async fn create_user(
    Valid(Json(payload)): Valid<Json<CreateUserRequest>>,
) -> Result<Json<User>, ApiError> {
    let user = db::create_user(payload).await?;
    Ok(Json(user))
}
```

---

### Step 6: Shared Application State

```rust
use axum::extract::State;
use sqlx::PgPool;
use std::sync::Arc;

// Application state — shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
    pub redis: redis::Client,
}

// Pass state to router
fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .with_state(state)
}

// Extract state in handlers
async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(users))
}
```

---

### Step 7: Middleware with Tower

```rust
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
};
use std::time::Duration;

fn build_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(api_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB
                .layer(cors),
        )
        .with_state(state)
}
```

---

### Step 8: Authentication — JWT

```toml
[dependencies]
jsonwebtoken = "9"
axum-extra = { version = "0.9", features = ["typed-header"] }
```

```rust
use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::{headers::{Authorization, Bearer}, TypedHeader};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user id
    pub exp: usize,    // expiry timestamp
    pub role: String,
}

// Custom extractor that validates JWT
pub struct AuthUser(pub Claims);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| ApiError::Unauthorized)?;

        let claims = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::Unauthorized)?
        .claims;

        Ok(AuthUser(claims))
    }
}

// Use the extractor in any handler
async fn protected_route(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Profile>, ApiError> {
    let profile = db::find_profile(&claims.sub, &state.db).await?;
    Ok(Json(profile))
}
```

---

### Step 9: Database Integration with SQLx

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "time", "migrate"] }
```

```rust
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

// Initialize pool
async fn init_db(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;

    // Run migrations automatically
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

// Compile-time checked queries
async fn get_user_by_email(pool: &PgPool, email: &str) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email, created_at FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

// Transactions
async fn transfer_funds(
    pool: &PgPool,
    from: Uuid,
    to: Uuid,
    amount: i64,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query!("UPDATE accounts SET balance = balance - $1 WHERE id = $2", amount, from)
        .execute(&mut *tx).await?;

    sqlx::query!("UPDATE accounts SET balance = balance + $1 WHERE id = $2", amount, to)
        .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(())
}
```

---

## 🔴 Phase 3 — Production-Ready API (Week 4–6)

### Step 10: Configuration Management

```toml
[dependencies]
config = "0.14"
dotenvy = "0.15"
```

```rust
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub environment: Environment,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
```

---

### Step 11: Pagination and Filtering

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

fn default_page() -> u64 { 1 }
fn default_per_page() -> u64 { 20 }

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: i64,
}

async fn list_users(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse<User>>, ApiError> {
    let offset = (pagination.page - 1) * pagination.per_page;

    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        pagination.per_page as i64,
        offset as i64,
    )
    .fetch_all(&state.db)
    .await?;

    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await?
        .unwrap_or(0);

    Ok(Json(PaginatedResponse {
        data: users,
        total,
        page: pagination.page,
        per_page: pagination.per_page,
        total_pages: (total + pagination.per_page as i64 - 1) / pagination.per_page as i64,
    }))
}
```

---

### Step 12: Rate Limiting

```toml
[dependencies]
tower_governor = "0.4"
```

```rust
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

fn build_app(state: AppState) -> Router {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(10)        // 10 requests per second
            .burst_size(30)        // Allow burst of 30
            .finish()
            .unwrap()
    );

    Router::new()
        .route("/api/login", post(login))
        .layer(GovernorLayer { config: governor_conf })
        .with_state(state)
}
```

---

### Step 13: OpenAPI Documentation

```toml
[dependencies]
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "6", features = ["axum"] }
```

```rust
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
    ),
    security(("bearer_auth" = [])),
)]
async fn get_user(Path(id): Path<Uuid>) -> Result<Json<User>, ApiError> {
    todo!()
}

#[derive(OpenApi)]
#[openapi(
    paths(get_user, create_user, list_users),
    components(schemas(User, CreateUserRequest, ApiError)),
    security(("bearer_auth" = [])),
)]
struct ApiDoc;

fn build_app(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api_routes())
        .with_state(state)
}
```

---

### Step 14: Caching with Redis

```toml
[dependencies]
redis = { version = "0.25", features = ["tokio-comp"] }
```

```rust
use redis::AsyncCommands;

pub struct Cache {
    client: redis::Client,
}

impl Cache {
    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<T>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let raw: Option<String> = conn.get(key).await?;
        Ok(raw.as_deref().map(serde_json::from_str).transpose()?)
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) -> anyhow::Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let raw = serde_json::to_string(value)?;
        conn.set_ex(key, raw, ttl_secs).await?;
        Ok(())
    }
}

// Cache-aside pattern in a handler
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, ApiError> {
    let cache_key = format!("user:{id}");

    // Try cache first
    if let Some(user) = state.cache.get::<User>(&cache_key).await? {
        return Ok(Json(user));
    }

    // Miss — query database
    let user = db::find_user(id, &state.db).await?
        .ok_or_else(|| ApiError::NotFound(format!("{id}")))?;

    // Populate cache (TTL = 5 minutes)
    state.cache.set(&cache_key, &user, 300).await?;

    Ok(Json(user))
}
```

---

### Step 15: Structured Logging & Tracing

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-opentelemetry = "0.22"
opentelemetry = "0.21"
```

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer().json()) // JSON for production
        .init();
}

// Add request IDs to logs
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use axum::http::HeaderName;

let x_request_id = HeaderName::from_static("x-request-id");

let middleware = ServiceBuilder::new()
    .layer(SetRequestIdLayer::new(x_request_id.clone(), MakeRequestUuid))
    .layer(PropagateRequestIdLayer::new(x_request_id))
    .layer(TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let request_id = request.headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");
            tracing::info_span!("request", request_id, method = %request.method(), uri = %request.uri())
        })
    );
```

---

### Step 16: Testing Your API

```rust
#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use super::*;

    async fn setup() -> TestServer {
        let state = AppState::test().await;
        let app = create_router(state);
        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    async fn test_health_check() {
        let server = setup().await;
        let res = server.get("/health").await;
        res.assert_status_ok();
        res.assert_text("OK");
    }

    #[tokio::test]
    async fn test_create_user() {
        let server = setup().await;

        let res = server
            .post("/users")
            .json(&json!({ "name": "Alice", "email": "alice@example.com", "age": 30 }))
            .await;

        res.assert_status(StatusCode::CREATED);
        let user: User = res.json();
        assert_eq!(user.name, "Alice");
    }

    #[tokio::test]
    async fn test_unauthorized() {
        let server = setup().await;
        let res = server.get("/api/profile").await;
        res.assert_status(StatusCode::UNAUTHORIZED);
    }
}
```

---

### Step 17: Deployment & Docker

```dockerfile
# Dockerfile — multi-stage build
FROM rust:1.79-slim AS builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build actual code
COPY src ./src
COPY migrations ./migrations
COPY config ./config
RUN touch src/main.rs && cargo build --release

# Minimal runtime image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/my-api .
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/config ./config

EXPOSE 3000
CMD ["./my-api"]
```

---

## ✅ Final API Checklist

Before shipping to production, verify:

- [ ] **Errors**: All errors map to appropriate HTTP status codes
- [ ] **Validation**: Request bodies are validated before processing
- [ ] **Auth**: JWT extraction and verification working
- [ ] **Database**: Connection pooling configured; migrations applied
- [ ] **Middleware**: CORS, tracing, compression, timeouts applied
- [ ] **Rate Limiting**: Sensitive routes protected
- [ ] **Docs**: OpenAPI/Swagger UI accessible at `/docs`
- [ ] **Caching**: Hot paths cached with appropriate TTL
- [ ] **Logging**: Structured JSON logs with request IDs
- [ ] **Tests**: Unit and integration tests passing
- [ ] **Docker**: Multi-stage build producing minimal image
- [ ] **Graceful Shutdown**: Server drains connections on SIGTERM

---

## 📚 Essential Resources

### Books & Guides

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Zero To Production In Rust](https://www.zero2prod.com/) ⭐ Best API book

### Crates to Know

| Crate                  | Purpose                          |
| ---------------------- | -------------------------------- |
| `axum`                 | Web framework                    |
| `sqlx`                 | Async SQL (compile-time checked) |
| `serde` + `serde_json` | Serialization                    |
| `tokio`                | Async runtime                    |
| `tower` + `tower-http` | Middleware                       |
| `tracing`              | Structured logging               |
| `anyhow` + `thiserror` | Error handling                   |
| `validator`            | Request validation               |
| `jsonwebtoken`         | JWT auth                         |
| `redis`                | Caching                          |
| `utoipa`               | OpenAPI generation               |
| `config`               | Configuration management         |
| `dotenvy`              | `.env` file loading              |

---

_Last updated: July 2026 • Rust 1.79+ • Axum 0.7+ • Tokio 1.x_
