fn find_winner(board: &Vec<Vec<char>>) -> Option<char> {
    let rows = board.len();
    if rows == 0 {
        return None;
    }
    let cols = board[0].len();

    let is_valid = |c: char| c != ' ';

    // Check Rows
    for r in 0..rows {
        let first = board[r][0];
        if is_valid(first) && board[r].iter().all(|&c| c == first) {
            return Some(first);
        }
    }

    // Check Columns
    for c in 0..cols {
        let first = board[0][c];
        if is_valid(first) && (1..rows).all(|r| board[r][c] == first) {
            return Some(first);
        }
    }

    // Check Main Diagonal (top-left to bottom-right)
    if rows == cols {
        let first = board[0][0];
        if is_valid(first) && (1..rows).all(|i| board[i][i] == first) {
            return Some(first);
        }

        // Check Anti-Diagonal (top-right to bottom-left)
        let first = board[0][cols - 1];
        if is_valid(first) && (1..rows).all(|i| board[i][cols - 1 - i] == first) {
            return Some(first);
        }
    }

    None
}

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['X', 'X', 'O'],
        vec!['X', ' ', ' '],
        vec!['X', 'X', 'O'],
    ];
    for x in &board {
        for y in x {
            print!("{}\t", y);
        }
        println!("");
    }
    let winner = find_winner(&board);
    dbg!(winner);
}
