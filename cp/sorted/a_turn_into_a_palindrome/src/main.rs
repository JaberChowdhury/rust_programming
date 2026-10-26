use std::io::{self, Read};

fn test(n: usize, c: char, s: &str) {
    let chars: Vec<char> = s.chars().collect();
    let mut count = 0;

    for i in 0..n / 2 {
        let left = chars[i];
        let right = chars[n - 1 - i];

        if left == right {
            continue;
        }

        if left == c || right == c {
            count += 1;
        } else {
            count += 2;
        }
    }

    println!("{}", count);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let c: char = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap();

        test(n, c, s);
    }
}
