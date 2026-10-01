use std::io;

fn case() -> bool {
    let mut valu_str = String::new();
    io::stdin().read_line(&mut valu_str).unwrap();
    let value: usize = valu_str.trim().parse().unwrap();

    if value & (value - 1) == 0 {
        false
    } else {
        true
    }
}
fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();

    let n = n_str.trim().parse::<u32>().unwrap();

    for _ in 0..n {
        let divisor = case();
        if divisor {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
