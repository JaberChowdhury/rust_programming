use crate::core::{Board, Player};
use crate::state::App;
use crate::utils::format_number;
use std::fs::File;
use std::io::Write;

fn render_board_md(board: &Board, n: usize) -> String {
    let mut s = String::new();
    s.push_str("```text\n");
    for r in 0..n {
        for c in 0..n {
            let p = board[r * n + c];
            s.push_str(&format!(" {} ", p.to_char()));
            if c < n - 1 {
                s.push('|');
            }
        }
        s.push('\n');
        if r < n - 1 {
            for c in 0..n {
                s.push_str("---");
                if c < n - 1 {
                    s.push('+');
                }
            }
            s.push('\n');
        }
    }
    s.push_str("```");
    s
}

pub fn write_report(app: &mut App) {
    if app.report_written {
        return;
    }
    app.report_written = true;

    let mut content = String::new();
    content.push_str("# 🎮 Tic-Tac-Toe Game Report\n\n");
    content.push_str("---\n\n");

    content.push_str("## 📊 Game Summary\n\n");
    content.push_str(&format!("- **Board Size:** {}x{}\n", app.n, app.n));
    content.push_str(&format!("- **Total Moves:** {}\n", app.history.len()));

    let total_evals: usize = app.history.iter().filter_map(|l| l.evals).sum();
    if total_evals > 0 {
        content.push_str(&format!(
            "- **Total AI States Evaluated:** {}\n",
            format_number(total_evals)
        ));
    }

    match app.winner {
        Some(Player::X) => content.push_str("- **Result:** 🏆 You (X) Won!\n\n"),
        Some(Player::O) => content.push_str("- **Result:** 🤖 AI (O) Won!\n\n"),
        None => content.push_str("- **Result:** 🤝 Draw\n\n"),
        _ => {}
    }

    content.push_str("---\n\n");

    content.push_str("## 🏁 Final Board\n\n");
    content.push_str(&render_board_md(&app.board, app.n));
    content.push_str("\n\n---\n\n");

    content.push_str("## ⏱️ Timelapse (Move by Move)\n\n");

    let mut timelapse_board = vec![Player::Empty; app.n * app.n];
    for (i, log) in app.history.iter().enumerate() {
        timelapse_board[log.position.0 * app.n + log.position.1] = log.player;

        let player_name = if log.player == Player::X {
            "You (X)"
        } else {
            "AI (O)"
        };
        content.push_str(&format!(
            "### Move {}: {} plays at ({}, {})\n\n",
            i + 1,
            player_name,
            log.position.0 + 1,
            log.position.1 + 1
        ));

        if let Some(score) = log.score {
            content.push_str(&format!("- **Move Score:** {}\n", score));
        }
        if let Some(evals) = log.evals {
            content.push_str(&format!(
                "- **States Evaluated:** {}\n",
                format_number(evals)
            ));
        }
        if log.score.is_some() || log.evals.is_some() {
            content.push_str("\n");
        }
        if !log.ai_logs.is_empty() {
            content.push_str("#### AI Move Evaluations:\n\n");
            content.push_str("| Status | Move (r, c) | Score | Reasoning |\n");
            content.push_str("|---|---|---|---|\n");
            let mut logs = log.ai_logs.clone();
            logs.reverse(); // Chronological order
            for (idx, score, reason, is_best) in logs {
                let r = idx / app.n;
                let c = idx % app.n;
                let is_final = (r, c) == log.position;
                let status = if is_final {
                    "**FINAL CHOICE**"
                } else if is_best {
                    "*ACCEPTED (Temp)*"
                } else {
                    "REJECTED"
                };
                content.push_str(&format!(
                    "| {} | ({}, {}) | {} | {} |\n",
                    status,
                    r + 1,
                    c + 1,
                    score,
                    reason
                ));
            }
            content.push_str("\n");
        }
        if let Some(reason) = &log.ai_reason {
            content.push_str(&format!("> **Final AI Reasoning:** {}\n\n", reason));
        }

        content.push_str(&render_board_md(&timelapse_board, app.n));
        content.push_str("\n\n");
    }

    if let Ok(mut file) = File::create("game_report.md") {
        let _ = file.write_all(content.as_bytes());
    }
}
