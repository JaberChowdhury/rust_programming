import re
import os

with open("src/main.rs", "r") as f:
    lines = f.readlines()

def get_section(start_marker, end_marker):
    start = -1
    end = -1
    for i, line in enumerate(lines):
        if start_marker in line:
            start = i
        elif end_marker in line and start != -1:
            end = i
            break
    if end == -1: end = len(lines)
    return "".join(lines[start:end])

# Extract sections
s_consts = get_section("Game constants", "Spatial grid")
s_grid1 = get_section("Spatial grid", "Snake state")
s_snake = get_section("Snake state", "Food grid (clustering)")
s_grid2 = get_section("Food grid (clustering)", "Camera")
s_ui1 = get_section("Camera", "UI helpers")
s_ui2 = get_section("UI helpers", "Main")
s_main = get_section("Main", "EOF_MARKER_NEVER_MATCH")

def make_pub(code):
    code = re.sub(r"^const ", "pub const ", code, flags=re.MULTILINE)
    code = re.sub(r"^struct ", "pub struct ", code, flags=re.MULTILINE)
    code = re.sub(r"^enum ", "pub enum ", code, flags=re.MULTILINE)
    code = re.sub(r"^fn ", "pub fn ", code, flags=re.MULTILINE)
    # Make struct fields pub
    # A bit hacky but works for this specific codebase
    # Replace lines inside struct that look like `name: Type,` with `pub name: Type,`
    new_lines = []
    in_struct = False
    for line in code.split("\n"):
        if line.startswith("pub struct"):
            in_struct = True
        elif in_struct and line.strip() == "}":
            in_struct = False
        
        if in_struct and ":" in line and not line.strip().startswith("pub") and not line.strip().startswith("//"):
            line = re.sub(r"^(\s+)([a-zA-Z_]+:)", r"\1pub \2", line)
            
        new_lines.append(line)
    return "\n".join(new_lines)

consts_code = "use macroquad::prelude::*;\n\n" + make_pub(s_consts)
grid_code = "use macroquad::prelude::*;\nuse crate::constants::*;\n\n" + make_pub(s_grid1 + s_grid2)
snake_code = "use macroquad::prelude::*;\nuse crate::constants::*;\nuse crate::grid::*;\n\n" + make_pub(s_snake)
ui_code = "use macroquad::prelude::*;\nuse crate::constants::*;\nuse crate::snake::*;\n\n" + make_pub(s_ui1 + s_ui2)

main_code = """use macroquad::prelude::*;

mod constants;
mod grid;
mod snake;
mod ui;

use constants::*;
use grid::*;
use snake::*;
use ui::*;

""" + s_main.replace("#[macroquad::main(window_conf)]", "#[macroquad::main(crate::ui::window_conf)]")

# Write files
with open("src/constants.rs", "w") as f: f.write(consts_code)
with open("src/grid.rs", "w") as f: f.write(grid_code)
with open("src/snake.rs", "w") as f: f.write(snake_code)
with open("src/ui.rs", "w") as f: f.write(ui_code)
with open("src/main.rs", "w") as f: f.write(main_code)

print("Refactor complete")
