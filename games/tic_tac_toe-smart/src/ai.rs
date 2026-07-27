//! This module contains all the Artificial Intelligence (AI) logic for the game.
//! It uses a combination of the Minimax algorithm, Dynamic Programming (Memoization),
//! and Multithreading to find the best possible move efficiently.

use crate::core::{AiEvent, Board, Player, check_winner, is_draw};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::Ordering};

/// The Minimax algorithm evaluates all possible future game states.
pub fn minimax(
    board: &mut Board,
    depth: i32,
    is_maximizing: bool,
    n: usize,
    memo: &mut HashMap<Board, i32>,
    states_evaluated: &std::sync::atomic::AtomicUsize,
) -> i32 {
    states_evaluated.fetch_add(1, Ordering::Relaxed);

    if let Some(&score) = memo.get(board) {
        return score;
    }

    if let Some(winner) = check_winner(board, n) {
        return match winner {
            Player::O => 100 - depth,
            Player::X => depth - 100,
            Player::Empty => 0,
        };
    }

    if is_draw(board, n) {
        return 0;
    }

    let result = if is_maximizing {
        let mut best_score = i32::MIN;
        for i in 0..(n * n) {
            if board[i] == Player::Empty {
                board[i] = Player::O;
                let score = minimax(board, depth + 1, false, n, memo, states_evaluated);
                board[i] = Player::Empty;
                best_score = best_score.max(score);
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for i in 0..(n * n) {
            if board[i] == Player::Empty {
                board[i] = Player::X;
                let score = minimax(board, depth + 1, true, n, memo, states_evaluated);
                board[i] = Player::Empty;
                best_score = best_score.min(score);
            }
        }
        best_score
    };

    memo.insert(board.clone(), result);
    result
}

/// This function calculates the absolute best move using multithreading,
/// and streams detailed explanations (Accepted/Rejected reasons) back to the UI.
pub fn get_best_move(
    board: &mut Board,
    n: usize,
    states_evaluated: Arc<std::sync::atomic::AtomicUsize>,
    tx_events: std::sync::mpsc::Sender<AiEvent>,
) {
    let mut depth = 0;
    let mut moves = Vec::new();

    for (i, &p) in board.iter().enumerate() {
        if p != Player::Empty {
            depth += 1;
        } else {
            moves.push(i);
        }
    }

    let moves_queue = Arc::new(Mutex::new(moves));
    let (tx_internal, rx_internal) = std::sync::mpsc::channel();
    let num_threads = std::cmp::min(10, board.len() - depth as usize);

    for _ in 0..num_threads {
        let queue = moves_queue.clone();
        let tx_internal = tx_internal.clone();
        let tx_events = tx_events.clone();
        let mut local_board = board.clone();
        let states_evaluated = states_evaluated.clone();

        std::thread::spawn(move || {
            let mut memo = HashMap::new();
            loop {
                let move_idx = {
                    let mut q = queue.lock().unwrap();
                    q.pop()
                };

                if let Some(i) = move_idx {
                    // Stream an event to the UI saying we are starting to evaluate this
                    let _ = tx_events.send(AiEvent::Evaluating(i));

                    local_board[i] = Player::O;
                    let score = minimax(
                        &mut local_board,
                        depth + 1,
                        false,
                        n,
                        &mut memo,
                        &states_evaluated,
                    );
                    local_board[i] = Player::Empty;

                    // Send the score internally to the controller
                    let _ = tx_internal.send((i, score));
                } else {
                    break;
                }
            }
        });
    }

    drop(tx_internal);

    let mut best_score = i32::MIN;
    let mut best_move = 0;
    let mut is_first = true;

    // As threads finish evaluating moves, we reason about them and send explanations to the UI
    for (m, score) in rx_internal {
        let is_best = is_first || score > best_score;

        let moves_ahead = if score > 0 {
            100 - score - depth
        } else if score < 0 {
            score + 100 - depth
        } else {
            0
        };

        let explanation = if score > 0 {
            if moves_ahead == 1 {
                "Instantly wins the game.".to_string()
            } else {
                format!("Forces a guaranteed win in {} moves.", moves_ahead)
            }
        } else if score == 0 {
            "Leads to a draw with optimal play.".to_string()
        } else {
            format!("Unavoidable loss in {} moves.", moves_ahead)
        };

        let reason = if is_best {
            format!("{} (Best option so far)", explanation)
        } else {
            if score < best_score {
                format!(
                    "{} Rejected: worse than best move ({} < {}).",
                    explanation, score, best_score
                )
            } else {
                format!("{} Skipped: equal to current best.", explanation)
            }
        };

        if is_best {
            best_score = score;
            best_move = m;
            is_first = false;
        }

        // Stream the detailed evaluation back to the UI
        let _ = tx_events.send(AiEvent::Evaluated {
            idx: m,
            score,
            best_so_far: is_best,
            reason,
        });
    }

    // Tell the UI we have conclusively picked the best move!
    let _ = tx_events.send(AiEvent::Finished(best_move));
}
