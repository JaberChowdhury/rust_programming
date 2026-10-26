use std::fmt::Debug;
use std::io;
use std::str::FromStr;

fn read_line_usr<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}

fn main() {
    let t: usize = read_line_usr()[0];
    for _ in 0..t {
        test();
    }
}

fn test() {
    let nk: Vec<i32> = read_line_usr();
    let n = &nk[0];
    let k = &nk[1];
    if n == k {
        println!("{}", n * 2);
    } else {
        let base = 2.0_f64;
        let result = ((k - 1) * 2) as f64 + base.powi(n - (k - 1));
        println!("{}", result);
    }
}
