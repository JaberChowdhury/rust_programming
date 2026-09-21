use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut it = input.split_ascii_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    for _ in 0..t {
        solve(&mut it, &mut out);
    }
}

fn solve<'a, I: Iterator<Item = &'a str>>(it: &mut I, out: &mut impl Write) {
    let mut x: u32;
    io::std
    // read input like:
    // let n: usize = it.next().unwrap().parse().unwrap();

    // write output like:
    // writeln!(out, "{}", n).unwrap();
}
