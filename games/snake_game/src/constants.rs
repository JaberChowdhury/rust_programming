use macroquad::prelude::*;

// ── Game constants ────────────────────────────────────────────────────────────
pub const SNAKE_SPEED: f32 = 100.0;
pub const TURN_SPEED: f32 = 5.2;
pub const FOOD_RADIUS: f32 = 3.0;
pub const MAX_SNAKES: usize = 50;
pub const MIN_FOOD: usize = 6400;
pub const OBSTACLE_COUNT: usize = 40;
pub const RAY_COUNT: usize = 16;
pub const RAY_LENGTH: f32 = 180.0;
pub const RAY_STEPS: usize = 7;
pub const HUNT_RANGE: f32 = 900.0;
pub const THREAT_RANGE: f32 = 300.0;
pub const SIZE_THREAT_MARGIN: usize = 0;
pub const MIN_HUNT_LENGTH: usize = 8;
pub const PREDICT_SECS: f32 = 0.35;
pub const FLEE_SPEED_BOOST: f32 = 1.6;
pub const HUNT_SPEED_BOOST: f32 = 1.35;
pub const CLUSTER_BONUS_RADIUS: f32 = 45.0;
pub const CLUSTER_BONUS_WEIGHT: f32 = 12.0;
pub const GRID_CELL: f32 = 28.0;

pub const ARENA_W: f32 = 6000.0;
pub const ARENA_H: f32 = 6000.0;

// ── Body-food chain reaction ──────────────────────────────────────────────────
// Head food lives BASE seconds, each subsequent piece gets CHAIN_DELAY extra.
pub const BODY_FOOD_BASE: f32 = 30.0; // seconds before head food vanishes
pub const BODY_FOOD_FADE: f32 = 0.55; // fade-out duration at end of life

// ── Camera ────────────────────────────────────────────────────────────────────
pub const FOLLOW_ZOOM: f32 = 2.8;

// ── UI Layout ─────────────────────────────────────────────────────────────────
pub const PANEL_W: f32 = 280.0;
pub const PANEL_PAD: f32 = 8.0;
pub const HEADER_H: f32 = 44.0;
pub const BTN_H: f32 = 34.0;
pub const ROW_H: f32 = 50.0;
pub const MAX_RANKS: usize = 13;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Arena — Bot Simulator".to_string(),
        window_width: 3560,
        window_height: 2000,
        fullscreen: true,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

pub const BG_COLOR: Color = Color::new(0.141, 0.153, 0.227, 1.0);
pub const FOOD_COLORS: [Color; 6] = [
    Color::new(0.957, 0.545, 0.659, 1.0),
    Color::new(0.651, 0.890, 0.631, 1.0),
    Color::new(0.537, 0.706, 0.980, 1.0),
    Color::new(0.976, 0.890, 0.686, 1.0),
    Color::new(0.796, 0.651, 0.969, 1.0),
    Color::new(0.580, 0.886, 0.835, 1.0),
];

pub const DUMMY_NAMES: [&str; 60] = [
    "Alex", "Sam", "Jordan", "Taylor", "Casey", "Morgan", "Riley", "Charlie",
    "Avery", "Parker", "Quinn", "Blake", "Dakota", "Reese", "Rowan", "Hayden",
    "Emerson", "Finley", "Harley", "Peyton", "River", "Skyler", "Spencer", "Tatum",
    "Cameron", "Dallas", "Drew", "Eden", "Elliott", "Frankie", "Hunter", "Jesse",
    "Kendall", "Lennon", "Micah", "Oakley", "Phoenix", "Rory", "Sage", "Sutton",
    "Noodle", "Slither", "Viper", "Cobra", "Python", "Mamba", "Danger Noodle",
    "Slytherin", "Medusa", "Ouroboros", "Basilisk", "Leviathan", "Jörmungandr",
    "Snek", "Long Boi", "Hiss", "Fangs", "Scales", "Squeeze", "Wraith",
];
