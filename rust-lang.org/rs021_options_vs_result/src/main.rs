use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Add lifetime 'a to connect the input vector's strings to the output
fn find_user<'a>(users: &'a Vec<&'a str>, _name: &str) -> Option<&'a str> {
    for &user in users {
        if user == _name {
            return Some(user);
        }
    }
    None
}

fn main() {
    match read_file_contents("config.txt") {
        Ok(data) => println!("Config loaded: {}", data),
        Err(e) => eprintln!("Failed to load config: {}", e),
    }

    let users = vec!["Alice", "Bob"];
    match find_user(&users, "Charlie") {
        Some(name) => println!("Found: {}", name),
        None => println!("User not found (normal outcome)."),
    }
}
