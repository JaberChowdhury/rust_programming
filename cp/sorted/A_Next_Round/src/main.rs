use std::io;

fn main() {
    let mut nk_string = String::new();
    io::stdin()
        .read_line(&mut nk_string)
        .expect("Failed to read");
    let nk: Vec<usize> = nk_string
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let _n = nk[0];
    let k = nk[1];

    let mut marks_string = String::new();
    io::stdin()
        .read_line(&mut marks_string)
        .expect("Failed to read");
    let scores: Vec<i32> = marks_string
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let threshold = scores[k - 1];

    let mut count = 0;
    for &score in &scores {
        if score >= threshold && score > 0 {
            count += 1;
        }
    }

    println!("{count}");
}
