use std::io;

fn take_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    n
}
fn take_string() -> &str {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim();
    n
}

fn main() {
    let n = take_number() as u32;
}
