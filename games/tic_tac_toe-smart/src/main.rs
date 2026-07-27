pub mod ai;
pub mod core;
pub mod report;
pub mod state;
pub mod ui;
pub mod utils;

use ai::get_best_move;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    error::Error,
    io,
    sync::{atomic::Ordering, mpsc},
    thread,
    time::Duration,
};

use crate::core::{AiEvent, Player};
use crate::report::write_report;
use crate::state::{App, AppState, TurnLog};
use crate::ui::ui;

impl App {
    pub fn check_game_over(&mut self) {
        if let Some(winner) = crate::core::check_winner(&self.board, self.n) {
            self.winner = Some(winner);
            self.state = AppState::GameOver;
            write_report(self);
        } else if crate::core::is_draw(&self.board, self.n) {
            self.winner = None;
            self.state = AppState::GameOver;
            write_report(self);
        }
    }

    pub fn start_ai_turn(&mut self) {
        self.state = AppState::AiTurn;
        self.board_before_ai = self.board.clone();

        // Reset stats for the new turn
        self.states_evaluated.store(0, Ordering::SeqCst);
        self.evaluating.clear();
        self.logs.clear();
        self.best_move_so_far = None;
        self.log_state.select(Some(0));

        let (tx, rx) = mpsc::channel();
        self.ai_event_rx = Some(rx);

        let mut board_clone = self.board.clone();
        let n = self.n;
        let states_evaluated = self.states_evaluated.clone();

        thread::spawn(move || {
            get_best_move(&mut board_clone, n, states_evaluated, tx);
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        // --------------------------------------------------
        // BACKGROUND AI LISTENER
        // --------------------------------------------------
        if app.state == AppState::AiTurn {
            if let Some(rx) = app.ai_event_rx.take() {
                // Drain all available messages from the AI threads instantly
                while let Ok(event) = rx.try_recv() {
                    match event {
                        AiEvent::Evaluating(idx) => {
                            app.evaluating.push(idx);
                        }
                        AiEvent::Evaluated {
                            idx,
                            score,
                            best_so_far,
                            reason,
                        } => {
                            app.evaluating.retain(|&x| x != idx);
                            let reason_clone = reason.clone();
                            // Push to the front so newest is at the top!
                            app.logs.insert(0, (idx, score, reason, best_so_far));
                            if best_so_far {
                                app.best_move_so_far = Some((idx, score, reason_clone));
                            }
                        }
                        AiEvent::Finished(best_move) => {
                            let ai_reason = app.best_move_so_far.as_ref().map(|b| b.2.clone());
                            let score = app.best_move_so_far.as_ref().map(|b| b.1);
                            let evals = app.states_evaluated.load(Ordering::Relaxed);
                            app.history.push(TurnLog {
                                player: Player::O,
                                position: (best_move / app.n, best_move % app.n),
                                ai_reason,
                                score,
                                evals: Some(evals),
                                ai_logs: app.logs.clone(),
                            });
                            app.board[best_move] = Player::O;
                            app.state = AppState::PlayerTurn;
                            app.check_game_over();
                        }
                    }
                }
                // Put the receiver back if the AI turn is not over yet
                if app.state == AppState::AiTurn {
                    app.ai_event_rx = Some(rx);
                }
            }
        }

        // --------------------------------------------------
        // KEYBOARD EVENT LISTENER
        // --------------------------------------------------
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match app.state {
                    AppState::Setup => match key.code {
                        KeyCode::Char(c) if c.is_ascii_digit() => app.input_buffer.push(c),
                        KeyCode::Backspace => {
                            app.input_buffer.pop();
                        }
                        KeyCode::Enter => {
                            if let Ok(n) = app.input_buffer.parse::<usize>() {
                                if n >= 3 {
                                    app.n = n;
                                    app.init_board();
                                } else {
                                    app.input_buffer.clear();
                                }
                            }
                        }
                        KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    },
                    AppState::PlayerTurn => {
                        let n = app.n;
                        match key.code {
                            KeyCode::Left => {
                                if app.cursor % n > 0 {
                                    app.cursor -= 1;
                                }
                            }
                            KeyCode::Right => {
                                if app.cursor % n < n - 1 {
                                    app.cursor += 1;
                                }
                            }
                            KeyCode::Up => {
                                if app.cursor >= n {
                                    app.cursor -= n;
                                }
                            }
                            KeyCode::Down => {
                                if app.cursor + n < n * n {
                                    app.cursor += n;
                                }
                            }
                            // Log scrolling
                            KeyCode::PageUp => {
                                let i = match app.log_state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            0
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.log_state.select(Some(i));
                            }
                            KeyCode::PageDown => {
                                let max = app.evaluating.len() + app.logs.len();
                                let i = match app.log_state.selected() {
                                    Some(i) => {
                                        if max > 0 && i >= max - 1 {
                                            max - 1
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.log_state.select(Some(i));
                            }
                            // Play piece
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                if app.board[app.cursor] == Player::Empty {
                                    app.history.push(TurnLog {
                                        player: Player::X,
                                        position: (app.cursor / app.n, app.cursor % app.n),
                                        ai_reason: None,
                                        score: None,
                                        evals: None,
                                        ai_logs: vec![],
                                    });

                                    app.board[app.cursor] = Player::X;
                                    app.check_game_over();
                                    if app.state != AppState::GameOver {
                                        app.start_ai_turn();
                                    }
                                }
                            }
                            KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        }
                    }
                    AppState::AiTurn => {
                        match key.code {
                            // Allow scrolling the logs even while the AI is thinking!
                            KeyCode::PageUp => {
                                let i = match app.log_state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            0
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.log_state.select(Some(i));
                            }
                            KeyCode::PageDown => {
                                let max = app.evaluating.len() + app.logs.len();
                                let i = match app.log_state.selected() {
                                    Some(i) => {
                                        if max > 0 && i >= max - 1 {
                                            max - 1
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.log_state.select(Some(i));
                            }
                            KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        }
                    }
                    AppState::GameOver => match key.code {
                        KeyCode::Char('r') => {
                            app.state = AppState::Setup;
                            app.input_buffer.clear();
                        }
                        KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    },
                }
            }
        }
    }
}
