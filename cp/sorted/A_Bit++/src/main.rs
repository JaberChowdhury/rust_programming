use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Please enter a valid integer");
    let mut count = 0;

    for _ in 0..num {
        let mut operation_string = String::new();
        io::stdin().read_line(&mut operation_string).expect("msg");
        let op = operation_string.contains("++");
        if op {
            count = count + 1;
        } else {
            count = count - 1;
        }
    }
    println!("{}", count);
}
