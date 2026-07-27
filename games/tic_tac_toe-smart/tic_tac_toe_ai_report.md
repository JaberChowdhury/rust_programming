# Advanced Multithreaded Tic-Tac-Toe AI

## Comprehensive Technical Report

This report documents the architecture, artificial intelligence logic, and execution flow of the custom **N×N Tic-Tac-Toe** game built using Rust, `ratatui`, and `crossterm`.

---

## 1. Project Overview

The project is a terminal-based, interactive Tic-Tac-Toe engine capable of playing perfectly on any `N×N` grid. It features a highly responsive text-user interface (TUI) that splits the screen into two halves:

- **Left Panel:** The interactive, geometric game board.
- **Right Panel:** A live-streaming "AI Insights" dashboard that visualizes the AI's internal thought process without freezing the user interface.

---

## 2. Artificial Intelligence Architecture

The AI is completely unbeatable. It utilizes a combination of mathematical game theory and modern systems programming paradigms to compute the absolute best move.

### 2.1 The Minimax Algorithm

The core of the AI is a recursive function called **Minimax**. Minimax simulates the game all the way to the end by assuming both players play perfectly.

- **Maximizing (AI's Turn):** The AI seeks the move that yields the highest possible score (`+100`).
- **Minimizing (User's Turn):** The algorithm simulates the user responding with the move that gives the AI the lowest possible score (`-100`).

To ensure the AI wins as quickly as possible (or delays losing as long as possible), the algorithm incorporates the current `depth` of the simulation into the score.

- _Win Score:_ `100 - depth`
- _Loss Score:_ `depth - 100`

### 2.2 Dynamic Programming (Memoization)

Tic-Tac-Toe has heavily overlapping subproblems (e.g., placing X then O results in the exact same board state as placing O then X). To prevent the AI from re-evaluating the same board state multiple times, we implemented **Memoization**.

- Each thread contains an isolated `HashMap<Board, i32>` cache.
- Before deeply evaluating a board, the thread checks the cache in O(1) time. If the exact board was evaluated previously, it instantly returns the cached score.

### 2.3 Multithreading & Work Stealing

For larger grids (e.g., 4x4), the branching factor explodes. We solved this by implementing a **Custom Thread Pool**:

- At the start of the AI's turn, it gathers all top-level empty cells and pushes them into an `Arc<Mutex<Vec<usize>>>` work queue.
- It spawns up to **10 concurrent worker threads**.
- Threads asynchronously lock the queue, "steal" a move to evaluate, and run the `minimax` simulation entirely in parallel.
- Because each thread uses an isolated memoization cache, there is absolutely zero locking overhead during the heavy recursive tree traversal.

---

## 3. UI and Concurrency

To prevent the UI from freezing while the AI searches millions of states, the engine utilizes asynchronous message passing.

### 3.1 Event Streaming (`mpsc`)

The worker threads stream their progress back to the main UI thread via a Multi-Producer, Single-Consumer (`mpsc`) channel using the `AiEvent` enum:

```rust
pub enum AiEvent {
    Evaluating(usize),
    Evaluated { idx: usize, score: i32, best_so_far: bool, reason: String },
    Finished(usize),
}
```

### 3.2 The AI Insights Dashboard

The UI instantly processes these incoming events at 20 frames per second:

1. **Stats Counter:** Displays the total number of board states evaluated across all threads (using a lock-free `AtomicUsize`).
2. **Best Move Mini-Board:** Dynamically renders a smaller version of the board highlighting the best path found.
3. **Scrollable Reasoning Log:** A historical log (navigable via `PageUp` / `PageDown`) explaining _why_ the AI made its choices.

---

## 4. Example Execution Log

Below is an example of what the AI Reasoning Log looks like under the hood during a typical 3x3 turn:

> **[Evaluating]** Thread analyzing move at row 0, col 0...
> **[Evaluating]** Thread analyzing move at row 1, col 1...
> **[Evaluating]** Thread analyzing move at row 2, col 2...
>
> **[REJECTED]** Move (0, 0) scored -98
>
> > _Reason:_ Score -98 is worse than current best (0).
>
> **[ACCEPTED]** Move (1, 1) scored 0
>
> > _Reason:_ Leads to a draw. Best option so far.
>
> **[ACCEPTED]** Move (0, 1) scored 99
>
> > _Reason:_ Leads to a winning position for the AI!
>
> **[REJECTED]** Move (2, 2) scored 99
>
> > _Reason:_ Score 99 is equal to best, skipping.

This architecture successfully marries deep algorithmic logic with high-performance concurrent Rust execution to deliver a heavily interactive educational experience!
