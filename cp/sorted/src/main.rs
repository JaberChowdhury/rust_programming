use std::io::{self, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut it = input.split_ascii_whitespace();

    solve(&mut it, &mut out);
}

fn solve<'a, I: Iterator<Item = &'a str>>(it: &mut I, out: &mut impl Write) {
    let w: u32 = it.next().unwrap().parse().unwrap();

    if w % 2 == 0 && w > 2 {
        writeln!(out, "YES").unwrap();
    } else {
        writeln!(out, "NO").unwrap();
    }
}
