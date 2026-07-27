use crate::core::{Board, Player};
use crate::state::{App, AppState};
use crate::utils::format_number;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListItem, Paragraph},
};
use std::sync::atomic::Ordering;

pub fn draw_board(
    f: &mut ratatui::Frame,
    area: Rect,
    board: &Board,
    n: usize,
    title: &str,
    highlight_idx: Option<usize>,
    highlight_color: Color,
) {
    let outer_block = Block::default().title(title).borders(Borders::ALL);
    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    let total_width = n as u16 * 6;
    let total_height = n as u16 * 3;

    let centered_h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(total_width),
            Constraint::Min(0),
        ])
        .split(inner_area)[1];

    let centered_v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(total_height),
            Constraint::Min(0),
        ])
        .split(centered_h)[1];

    let row_slices = Layout::default()
        .direction(Direction::Vertical)
        .constraints((0..n).map(|_| Constraint::Length(3)).collect::<Vec<_>>())
        .split(centered_v);

    for r in 0..n {
        let col_slices = Layout::default()
            .direction(Direction::Horizontal)
            .constraints((0..n).map(|_| Constraint::Length(6)).collect::<Vec<_>>())
            .split(row_slices[r]);

        for c in 0..n {
            let idx = r * n + c;
            let p = board[idx];

            let mut block = Block::default().borders(Borders::ALL);
            if Some(idx) == highlight_idx {
                block = block.style(Style::default().bg(highlight_color));
            }

            let (char_str, fg_color) = match p {
                Player::X => ("X", Color::LightBlue),
                Player::O => ("O", Color::LightRed),
                Player::Empty => ("", Color::Reset),
            };

            let p_text = Paragraph::new(char_str)
                .block(block)
                .alignment(Alignment::Center)
                .style(Style::default().fg(fg_color));

            f.render_widget(p_text, col_slices[c]);
        }
    }
}

pub fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    let left_chunk = chunks[0];
    let right_chunk = chunks[1];

    match app.state {
        AppState::Setup => {
            let p = Paragraph::new(format!("Enter board size N (>= 3): {}", app.input_buffer))
                .block(Block::default().title("Setup").borders(Borders::ALL))
                .alignment(Alignment::Center);
            f.render_widget(p, left_chunk);
        }
        AppState::PlayerTurn | AppState::AiTurn | AppState::GameOver => {
            let highlight = if app.state == AppState::PlayerTurn {
                Some(app.cursor)
            } else {
                None
            };
            draw_board(
                f,
                left_chunk,
                &app.board,
                app.n,
                " Tic-Tac-Toe ",
                highlight,
                Color::DarkGray,
            );

            let evals = app.states_evaluated.load(Ordering::Relaxed);

            let has_ai_history = app.history.iter().any(|h| !h.ai_logs.is_empty());
            if app.state == AppState::AiTurn || !app.logs.is_empty() || has_ai_history {
                let right_splits = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(4),      // Top stats panel
                        Constraint::Percentage(40), // Middle best-move board
                        Constraint::Percentage(60), // Bottom scrollable logs
                    ])
                    .split(right_chunk);

                // Top Status Panel
                let status_msg = if app.state == AppState::AiTurn {
                    " AI is thinking... "
                } else {
                    " AI Finished. Your turn! (Arrow keys + Enter to play) "
                };
                let status_color = if app.state == AppState::AiTurn {
                    Color::Yellow
                } else {
                    Color::Green
                };

                let p = Paragraph::new(vec![
                    Line::from(Span::styled(status_msg, Style::default().fg(status_color))),
                    Line::from(format!(" States Evaluated: {}", format_number(evals))),
                ])
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(p, right_splits[0]);

                let display_board = if app.state == AppState::AiTurn {
                    &app.board
                } else {
                    &app.board_before_ai
                };

                // Middle Panel: Best Move Found
                if let Some((idx, score, _)) = &app.best_move_so_far {
                    let mut best_board = display_board.clone();
                    best_board[*idx] = Player::O;
                    draw_board(
                        f,
                        right_splits[1],
                        &best_board,
                        app.n,
                        &format!(" Best Move Found (Score: {}) ", score),
                        Some(*idx),
                        Color::Green,
                    );
                } else {
                    let block = Block::default()
                        .title(" Best Move Found ")
                        .borders(Borders::ALL);
                    f.render_widget(block, right_splits[1]);
                }

                // Bottom Panel: Scrollable Reasoning Logs
                let mut list_items = Vec::new();

                // Show currently evaluating threads at the top
                for &idx in &app.evaluating {
                    let r = idx / app.n;
                    let c = idx % app.n;
                    let text = format!(
                        " [Evaluating] Thread analyzing move at row {}, col {}...",
                        r + 1,
                        c + 1
                    );
                    list_items.push(ListItem::new(text).style(Style::default().fg(Color::Yellow)));
                }

                // Show completed evaluations
                for log in &app.logs {
                    let (idx, score, reason, is_best) = log;
                    let r = idx / app.n;
                    let c = idx % app.n;

                    let status = if *is_best { "ACCEPTED" } else { "REJECTED" };
                    let color = if *is_best {
                        Color::Green
                    } else {
                        Color::DarkGray
                    };

                    let text = format!(
                        " [{}] Move ({}, {}) scored {}\n   Reason: {}",
                        status,
                        r + 1,
                        c + 1,
                        score,
                        reason
                    );
                    list_items.push(ListItem::new(text).style(Style::default().fg(color)));
                }

                // Show history
                for (turn_i, log) in app.history.iter().enumerate().rev() {
                    if !log.ai_logs.is_empty() {
                        let header = format!(" --- Turn {} (AI) ---", turn_i + 1);
                        list_items.push(
                            ListItem::new(header).style(
                                Style::default()
                                    .fg(Color::Cyan)
                                    .add_modifier(Modifier::BOLD),
                            ),
                        );

                        let logs = log.ai_logs.clone();
                        // Optional: chronological? In UI, newest is top, so we keep reverse chronological (no .reverse())
                        for (idx, score, reason, is_best) in logs {
                            let r = idx / app.n;
                            let c = idx % app.n;
                            let is_final = (r, c) == log.position;

                            let status = if is_final {
                                "FINAL CHOICE"
                            } else if is_best {
                                "ACCEPTED"
                            } else {
                                "REJECTED"
                            };
                            let color = if is_final {
                                Color::LightGreen
                            } else if is_best {
                                Color::Green
                            } else {
                                Color::DarkGray
                            };

                            let text = format!(
                                " [{}] Move ({}, {}) scored {}\n   Reason: {}",
                                status,
                                r + 1,
                                c + 1,
                                score,
                                reason
                            );
                            list_items.push(ListItem::new(text).style(Style::default().fg(color)));
                        }
                    }
                }

                let logs_list = ratatui::widgets::List::new(list_items)
                    .block(
                        Block::default()
                            .title(" AI Reasoning Logs ")
                            .borders(Borders::ALL),
                    )
                    .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

                f.render_stateful_widget(logs_list, right_splits[2], &mut app.log_state);
            }
        }
    }
}
