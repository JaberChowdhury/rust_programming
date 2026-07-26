use std::io;

enum Calculator {
    BMI,
    BMR,
}
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}
#[derive(Debug)]
struct Bmi {
    weight: f32,
    height: f32,
}
#[derive(Debug)]
struct Bmr {
    gender: Gender,
    weight: f32,
    height: f32,
    age: i32,
}
fn read_f32(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        match input.trim().parse::<f32>() {
            Ok(val) => return val,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
impl Bmi {
    fn take_input(&mut self) {
        self.weight = read_f32("Please enter your weight in kg :: ");
        self.height = read_f32("Please enter your height in meter :: ");
    }
    fn calculate(&self) -> f32 {
        let bmi = self.weight / (self.height * self.height);
        bmi
    }
    fn status(&self) {
        let bmi = self.calculate();
        dbg!(self, bmi);
    }
}
impl Bmr {
    fn take_input(&mut self) {
        loop {
            let g = read_f32("Please enter your gender \n1. Male\n2. female") as i32;
            match g {
                1 => {
                    self.gender = Gender::Male;
                    break;
                }
                2 => {
                    self.gender = Gender::Female;
                    break;
                }
                _ => {
                    println!("Wrong input");
                    continue;
                }
            }
        }

        self.weight = read_f32("Please enter your weight in kg :: ");
        self.height = read_f32("Please enter your height in meter :: ");
        let age: i32 = read_f32("Please enter your age :: ") as i32;
        self.age = age;
    }
    fn calculate(&self) -> f32 {
        let bmr;
        match self.gender {
            Gender::Male => {
                bmr = (4.536 * self.weight) + (15.88 * self.height) - (5.0 * self.age as f32) + 5.0;
            }
            Gender::Female => {
                bmr =
                    (4.536 * self.weight) + (15.88 * self.height) - (5.0 * self.age as f32) - 161.0;
            }
        }
        bmr
    }
    fn status(&self) {
        let bmr = self.calculate();
        dbg!(self, bmr);
    }
}
fn calculator() {
    let input;
    loop {
        let user_input = read_f32("\nEnter your choise in number \n1. bmi\n2. bmr") as i32;
        match user_input {
            1 => {
                input = Calculator::BMI;
                break;
            }
            2 => {
                input = Calculator::BMR;
                break;
            }
            _ => {
                print!("Wrong input try again properly");
            }
        }
    }
    match input {
        Calculator::BMI => {
            let mut bmi = Bmi {
                weight: 12.21,
                height: 23.23,
            };
            bmi.take_input();
            bmi.status();
        }
        Calculator::BMR => {
            let mut bmr = Bmr {
                weight: 12.21,
                height: 23.23,
                age: 12,
                gender: Gender::Male,
            };
            bmr.take_input();
            bmr.status();
        }
    }
}
fn main() {
    calculator();
}
