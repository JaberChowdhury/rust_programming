fn get_element_at_index(vec: &Vec<i32>, index: usize) -> Option<i32> {
    // Returns Some(value) if index is valid, None if out of bounds
    vec.get(index).copied()
}

fn main() {
    let numbers = vec![10, 20, 30];

    // Valid index
    // if let Some(value) = get_element_at_index(&numbers, 1) {
    //     println!("Found: {}", value); // Output: Found: 20
    // }

    // Invalid index (not an error, just empty)
    match get_element_at_index(&numbers, 5) {
        Some(value) => println!("Found: {}", value),
        None => println!("Index out of bounds, no value found."),
    }
}
