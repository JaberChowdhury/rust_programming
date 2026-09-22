mod input;
mod math;

use input::take_input;
use math::*;

fn main() {
    println!("--- Simple Calculator ---");

    let num1: f64 = take_input("Enter the first number: ");
    let num2: f64 = take_input("Enter the second number: ");

    let operator: String = take_input("Enter operator (+, -, *, /): ");

    let result = match operator.as_str() {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Error: Unknown operator!");
            return;
        }
    };

    println!("\nResult: {} {} {} = {}", num1, operator, num2, result);
}
