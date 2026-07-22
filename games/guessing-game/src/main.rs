// use rand::Rng;
use std::io;

fn main() {
    let random_num = rand::random_range(0..100);
    println!("Guess a number between 0 and 100!");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("faild to take input");

        let guess: i32 = match user_input.trim().parse() {
            Ok(numx) => numx,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        if random_num == guess {
            println!("Congratulation");
            break;
        } else if random_num > guess {
            println!("Your input number is small");
        } else {
            println!("Your input number is big");
        }
    }
}
