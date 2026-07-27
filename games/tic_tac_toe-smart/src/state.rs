use crate::core::{AiEvent, Board, Player};
use ratatui::widgets::ListState;
use std::sync::{Arc, atomic::AtomicUsize, mpsc};

#[derive(PartialEq)]
pub enum AppState {
    Setup,
    PlayerTurn,
    AiTurn,
    GameOver,
}

pub struct TurnLog {
    pub player: Player,
    pub position: (usize, usize),
    pub ai_reason: Option<String>,
    pub score: Option<i32>,
    pub evals: Option<usize>,
    pub ai_logs: Vec<(usize, i32, String, bool)>,
}

pub struct App {
    pub state: AppState,
    pub n: usize,
    pub input_buffer: String,
    pub board: Board,
    pub board_before_ai: Board,
    pub cursor: usize,
    pub winner: Option<Player>,

    pub states_evaluated: Arc<AtomicUsize>,
    pub ai_event_rx: Option<mpsc::Receiver<AiEvent>>,

    pub evaluating: Vec<usize>,
    pub logs: Vec<(usize, i32, String, bool)>,
    pub best_move_so_far: Option<(usize, i32, String)>,
    pub log_state: ListState,

    pub history: Vec<TurnLog>,
    pub report_written: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Setup,
            n: 3,
            input_buffer: String::new(),
            board: vec![],
            board_before_ai: vec![],
            cursor: 0,
            winner: None,

            states_evaluated: Arc::new(AtomicUsize::new(0)),
            ai_event_rx: None,

            evaluating: vec![],
            logs: vec![],
            best_move_so_far: None,
            log_state: ListState::default(),

            history: vec![],
            report_written: false,
        }
    }

    pub fn init_board(&mut self) {
        self.board = vec![Player::Empty; self.n * self.n];
        self.cursor = 0;
        self.state = AppState::PlayerTurn;
        self.history.clear();
        self.report_written = false;
    }
}
