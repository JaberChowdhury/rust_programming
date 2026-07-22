use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone, PartialEq)]
enum GameState {
    Playing,
    Winner(Player),
    Draw,
}

struct TicTacToe {
    board: [Option<Player>; 9],
    current_turn: Player,
    state: GameState,
}

impl TicTacToe {
    fn new() -> Self {
        Self {
            board: [None; 9],
            current_turn: Player::X,
            state: GameState::Playing,
        }
    }

    fn play(&mut self, idx: usize) {
        if self.state != GameState::Playing || self.board[idx].is_some() {
            return;
        }

        self.board[idx] = Some(self.current_turn);

        if self.check_winner(self.current_turn) {
            self.state = GameState::Winner(self.current_turn);
        } else if self.board.iter().all(|x| x.is_some()) {
            self.state = GameState::Draw;
        } else {
            self.current_turn = match self.current_turn {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }
    }

    fn check_winner(&self, player: Player) -> bool {
        let win_patterns = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8], // Rows
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8], // Cols
            [0, 4, 8],
            [2, 4, 6], // Diagonals
        ];

        win_patterns
            .iter()
            .any(|pattern| pattern.iter().all(|&idx| self.board[idx] == Some(player)))
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = TicTacToe::new();

    loop {
        clear_background(Color::new(251.0 / 255.0, 208.0 / 255.0, 116.0 / 255.0, 1.0));

        let screen_size = screen_width().min(screen_height());
        let cell_size = screen_size / 3.0;
        let offset_x = (screen_width() - screen_size) / 2.0;
        let offset_y = (screen_height() - screen_size) / 2.0;

        // Draw Board
        for i in 1..3 {
            // Vertical lines
            draw_line(
                offset_x + i as f32 * cell_size,
                offset_y,
                offset_x + i as f32 * cell_size,
                offset_y + screen_size,
                5.0,
                DARKGRAY,
            );
            // Horizontal lines
            draw_line(
                offset_x,
                offset_y + i as f32 * cell_size,
                offset_x + screen_size,
                offset_y + i as f32 * cell_size,
                5.0,
                DARKGRAY,
            );
        }

        // Draw X and O
        for i in 0..9 {
            let row = i / 3;
            let col = i % 3;
            let x = offset_x + col as f32 * cell_size + cell_size / 2.0;
            let y = offset_y + row as f32 * cell_size + cell_size / 2.0;
            let padding = cell_size * 0.2;

            match game.board[i] {
                Some(Player::X) => {
                    draw_line(
                        x - cell_size / 2.0 + padding,
                        y - cell_size / 2.0 + padding,
                        x + cell_size / 2.0 - padding,
                        y + cell_size / 2.0 - padding,
                        8.0,
                        RED,
                    );
                    draw_line(
                        x + cell_size / 2.0 - padding,
                        y - cell_size / 2.0 + padding,
                        x - cell_size / 2.0 + padding,
                        y + cell_size / 2.0 - padding,
                        8.0,
                        RED,
                    );
                }
                Some(Player::O) => {
                    draw_circle_lines(x, y, cell_size / 2.0 - padding, 8.0, BLUE);
                }
                None => {}
            }
        }

        // Handle Input
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if game.state == GameState::Playing {
                if mx > offset_x
                    && mx < offset_x + screen_size
                    && my > offset_y
                    && my < offset_y + screen_size
                {
                    let col = ((mx - offset_x) / cell_size) as usize;
                    let row = ((my - offset_y) / cell_size) as usize;
                    let idx = row * 3 + col;
                    if idx < 9 {
                        game.play(idx);
                    }
                }
            } else {
                // Reset on click if game is over
                game.reset();
            }
        }

        // Draw Status
        match game.state {
            GameState::Playing => {}
            GameState::Winner(p) => {
                let text = match p {
                    Player::X => "X Wins! Click to restart",
                    Player::O => "O Wins! Click to restart",
                };
                let text_size = measure_text(text, None, 50, 1.0);
                draw_rectangle(
                    0.0,
                    screen_height() / 2.0 - 50.0,
                    screen_width(),
                    100.0,
                    Color::new(1.0, 1.0, 1.0, 0.8),
                );
                draw_text(
                    text,
                    screen_width() / 2.0 - text_size.width / 2.0,
                    screen_height() / 2.0 + text_size.height / 2.0,
                    50.0,
                    BLACK,
                );
            }
            GameState::Draw => {
                let text = "Draw! Click to restart";
                let text_size = measure_text(text, None, 50, 1.0);
                draw_rectangle(
                    0.0,
                    screen_height() / 2.0 - 50.0,
                    screen_width(),
                    100.0,
                    Color::new(1.0, 1.0, 1.0, 0.8),
                );
                draw_text(
                    text,
                    screen_width() / 2.0 - text_size.width / 2.0,
                    screen_height() / 2.0 + text_size.height / 2.0,
                    50.0,
                    BLACK,
                );
            }
        }

        next_frame().await
    }
}
