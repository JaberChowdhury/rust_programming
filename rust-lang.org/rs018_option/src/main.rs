#[derive(Debug)]
struct Person {
    first_name: String,
    middle_name: Option<String>, // Optional field
    last_name: String,
}

fn main() {
    let vec = vec![1, 2, 3];
    let element = vec.get(5); // Returns Option<&i32>
    let p = Person {
        first_name: "Jane".to_string(),
        middle_name: None, // Explicitly no middle name
        last_name: "Doe".to_string(),
    };
    dbg!(p);
    match element {
        Some(val) => println!("Found: {}", val),
        None => println!("Index out of bounds"), // Handles error gracefully
    }
}
fn find_char(text: &str, target: char) -> Option<usize> {
    for (index, ch) in text.chars().enumerate() {
        if ch == target {
            return Some(index);
        }
    }
    None // Target not found
}
