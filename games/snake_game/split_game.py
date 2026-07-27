import re

with open("src/main.rs", "r") as f:
    main_code = f.read()

game_code = main_code.replace("#[macroquad::main(window_conf)]\nasync fn main() {", "pub async fn run_game() {")
game_code = game_code.replace("use constants::*;\nuse grid::*;\nuse snake::*;\nuse ui::*;", "use crate::constants::*;\nuse crate::grid::*;\nuse crate::snake::*;\nuse crate::ui::*;")
# Remove mod declarations from game.rs
game_code = re.sub(r"mod constants;\nmod grid;\nmod snake;\nmod ui;\n", "", game_code)

new_main = """use macroquad::prelude::*;

mod constants;
mod grid;
mod snake;
mod ui;
mod game;

use constants::*;

#[macroquad::main(window_conf)]
async fn main() {
    game::run_game().await;
}
"""

with open("src/game.rs", "w") as f:
    f.write(game_code)

with open("src/main.rs", "w") as f:
    f.write(new_main)

