# Rust Transition Project Checklist

## Project 1: Command-Line Task Manager (Focus: Ownership, Structs, File I/O)

Phase 1: Environment and Core Data Structures
[ ] Initialize the project using cargo new rust_task_manager.

[ ] Define a Task struct in src/main.rs containing id (u32), description (String), and status (Enum: Pending/Completed).

[ ] Define a TaskManager struct that holds a Vec<Task>.

[ ] Implement an add_task method for TaskManager that takes ownership of a description string and pushes a new Task into the vector.

[ ] Implement a list_tasks method utilizing Rust iterators (.iter()) to print current tasks to the console.

Phase 2: Error Handling and Persistence
[ ] Add serde and serde_json to Cargo.toml dependencies.

[ ] Derive Serialize and Deserialize traits for the Task and TaskStatus structs.

[ ] Implement a save_to_disk method using std::fs::File and serde_json::to_writer.

[ ] Implement a load_from_disk method. Use the Result enum and the ? operator to safely handle cases where the file does not exist or is corrupted.

Phase 3: Command Line Parsing and Refactoring
[ ] Add the clap crate to Cargo.toml for CLI argument parsing.

[ ] Define a Cli struct with subcommands (e.g., Add, Complete, List, Delete) using clap's derive macros.

[ ] Implement a match statement in the main function to route the CLI subcommands to the corresponding TaskManager methods.

[ ] Refactor the codebase by moving the Task and TaskManager implementations into a separate task.rs module.

[ ] Run cargo clippy and resolve all linter warnings regarding idiomatic Rust styling.

### Project 2: Concurrent HTTP Server (Focus: Concurrency, Traits, Networking)

Phase 1: Basic TCP Communication
[ ] Initialize the project using cargo new rust_web_server.

[ ] Bind a TcpListener to 127.0.0.1:7878 using std::net::TcpListener::bind.

[ ] Implement a for loop over listener.incoming() to accept incoming TCP streams.

[ ] Write a handle_connection function that reads the stream using BufReader and prints the HTTP request headers to the console.

[ ] Modify handle_connection to write a standard HTTP 200 OK response with a basic HTML payload back to the stream.

Phase 2: Multithreading and Thread Pools
[ ] Create a ThreadPool struct capable of holding a vector of Worker instances.

[ ] Implement the mpsc::channel to allow the ThreadPool to send closures (jobs) to the workers.

[ ] Wrap the receiving end of the channel in Arc<Mutex<mpsc::Receiver<Job>>> so it can be safely shared across the worker threads.

[ ] Inside each Worker thread, implement an infinite loop that acquires the Mutex lock, receives a job from the channel, and executes the closure.

Phase 3: Integration and Optimization
[ ] Modify the main connection loop to pass the handle_connection closure to the ThreadPool.execute() method instead of running it synchronously.

[ ] Simulate heavy workloads by adding a std::thread::sleep delay to specific HTTP routes (e.g., /sleep) and verify that other routes remain responsive.

[ ] Implement graceful shutdown logic by utilizing the Drop trait on the ThreadPool to join all worker threads before the main process exits.
