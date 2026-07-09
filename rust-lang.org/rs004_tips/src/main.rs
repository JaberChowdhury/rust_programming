fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub_sequence_strings(data: &str) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let char_count = data.chars().count();

    for i in 0..char_count {
        for j in i + 1..=char_count {
            // let mut temp: String = "".to_string();
            let mut temp: Vec<String> = Vec::new();
            for k in i..j {
                if let Some(c) = data.chars().nth(k) {
                    temp.push(c.to_string());
                }
            }
            result.push(temp);
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
    let added = add(12, 21);
    println!("{added}");
    dbg!(add(32, 34));
    let text = String::from("programming");
    let result = sub_sequence_strings(&text);

    dbg!(result.len());
    dbg!(&result);

    todo!("Will be added later");
}
