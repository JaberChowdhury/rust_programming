// unused import removed

mod constants;
mod game;
mod grid;
mod shaders;
mod snake;
mod ui;

use constants::*;

#[macroquad::main(window_conf)]
async fn main() {
    game::run_game().await;
}
