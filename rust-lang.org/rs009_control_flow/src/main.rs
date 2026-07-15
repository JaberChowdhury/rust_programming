fn main() {
    let age = 11;
    let is_human = false;
    if is_human {
        if age <= 0 {
            println!("Invalid age");
        } else if age > 120 {
            println!("Whattt ??");
        } else if age > 60 {
            println!("Old");
        } else if age >= 18 {
            println!("Adult");
        } else if age < 18 {
            println!("Child");
        }
    } else {
        println!("You are not even a human");
    }

    let is_married = if age >= 18 {
        "may be"
    } else if age == 21 {
        "21"
    } else {
        "No"
    };
    dbg!(is_married);
    println!("Hello, world!");
}
