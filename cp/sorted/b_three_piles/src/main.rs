use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();

        let ans = ((a - b).abs()).max(((a + c) - b).abs());
        println!("{}", ans);
    }
}
