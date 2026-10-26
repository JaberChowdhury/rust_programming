// Problem: A. Good Contest
// URL: https://codeforces.com/problemset/problem/2266/A

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        let c: i32 = iter.next().unwrap().parse().unwrap();

        let ans = n - a.min(b).min(c);

        // Write your solution here
        println!("{}", ans);
    }
}
