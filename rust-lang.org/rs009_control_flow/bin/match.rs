enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn main() {
    move_to_path(Direction::Up);
    move_to_path(Direction::Left);
    move_to_path(Direction::Down);
    move_to_path(Direction::Right);
}

fn move_to_path(dir: Direction) {
    match dir {
        Direction::Up => println!("Go up"),
        Direction::Left => println!("Go left"),
        Direction::Down => println!("Go down"),
        Direction::Right => println!("Go right"),
    }
}
