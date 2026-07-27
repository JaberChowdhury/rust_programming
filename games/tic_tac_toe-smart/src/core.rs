#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Player {
    X,
    O,
    Empty,
}

impl Player {
    pub fn to_char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
            Player::Empty => ' ',
        }
    }
}

pub type Board = Vec<Player>;

pub fn check_winner(board: &Board, n: usize) -> Option<Player> {
    for r in 0..n {
        let first = board[r * n];
        if first != Player::Empty && (1..n).all(|c| board[r * n + c] == first) {
            return Some(first);
        }
    }
    for c in 0..n {
        let first = board[c];
        if first != Player::Empty && (1..n).all(|r| board[r * n + c] == first) {
            return Some(first);
        }
    }
    let first = board[0];
    if first != Player::Empty && (1..n).all(|i| board[i * n + i] == first) {
        return Some(first);
    }
    let first = board[n - 1];
    if first != Player::Empty && (1..n).all(|i| board[i * n + (n - 1 - i)] == first) {
        return Some(first);
    }
    None
}

pub fn is_draw(board: &Board, n: usize) -> bool {
    board.iter().all(|&p| p != Player::Empty) && check_winner(board, n).is_none()
}

/// The events sent from the multithreaded AI back to the main UI thread.
pub enum AiEvent {
    Evaluating(usize),
    Evaluated {
        idx: usize,
        score: i32,
        best_so_far: bool,
        reason: String,
    },
    Finished(usize),
}
