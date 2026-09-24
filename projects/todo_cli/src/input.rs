use std::{
    io::{self, Write},
    str::FromStr,
};

pub fn take_input<T>(prompt: &str) -> T
where
    T: FromStr,
{
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        match io::stdout().flush() {
            Ok(_) => { /* Everything is fine, move on */ }
            Err(error) => println!("Failed to flush to screen: {}", error),
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Fatal error: failed to read from stdin");
        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please try again.\n"),
        }
    }
}
