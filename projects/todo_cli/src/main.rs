use std::io;

use rand::RngExt;

#[derive(Debug)]
struct Todo {
    id: String,
    title: String,
    description: String,
    // is_deleted: bool,
}
fn random_id() -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let mut id = String::new();

    for i in 1..17 {
        let random_idx = rand::rng().random_range(0..charset.len());
        if let Some(random_char) = charset.chars().nth(random_idx) {
            id.push(random_char);
            if i % 4 == 0 && i < 16 {
                id.push('-');
            }
        }
    }
    id
}
fn add_todos(todos: &mut Vec<Todo>) {
    let mut input_title = String::new();
    let mut input_description = String::new();
    println!("Please enter a title :: ");
    io::stdin()
        .read_line(&mut input_title)
        .expect("Failed to take user input");
    println!("Please enter a description::");
    io::stdin()
        .read_line(&mut input_description)
        .expect("Failed to take user input");

    todos.push(Todo {
        id: random_id(),
        title: input_title.trim().to_string(),
        description: input_description.trim().to_string(),
    });
}
fn delete_todos(todos: &mut Vec<Todo>) {
    let mut id_input = String::new();
    println!("Please enter an id to delete the todo::");
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to take user input");
    let id = id_input.trim().to_string();
    let index_to_delete = todos.iter().position(|todo| todo.id == id);
    match index_to_delete {
        Some(index) => {
            todos.remove(index);
        }
        None => {
            println!("Todo not found");
        }
    }
}
fn update_todos(todos: &mut Vec<Todo>) {
    let mut id_input = String::new();
    // let mut input_title = String::new();
    // let mut input_description = String::new();
    println!("Please enter an valid id to update the todo:: ");
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to take user input");

    let id = id_input.trim().to_string();

    let id_for_update = todos.iter().position(|todo| todo.id == id);

    match id_for_update {
        Some(index) => {
            let mut input_title = String::new();
            let mut input_description = String::new();

            println!("Please enter a new title :: ");
            io::stdin()
                .read_line(&mut input_title)
                .expect("Failed to take user input");

            println!("Please enter a new description::");
            io::stdin()
                .read_line(&mut input_description)
                .expect("Failed to take user input");

            todos[index].title = input_title.trim().to_string();
            todos[index].description = input_description.trim().to_string();

            println!("Todo updated successfully!");
        }
        None => {
            println!("Todo not found");
        }
    }
}
fn print_todos(todos: &mut Vec<Todo>) {
    dbg!(todos);
}
fn todos_cli(todos: &mut Vec<Todo>) {
    loop {
        let mut input = String::new();
        println!(
            "\n\nChoose any valid operation from below\n1. Add todo\n2. Delete todo\n3. Show todos\n4. Update todo\n5. Quit"
        );
        io::stdin()
            .read_line(&mut input)
            .expect("something went wrong");
        let choice = input.trim().parse::<i32>().expect("msg");
        match choice {
            1 => add_todos(todos),
            2 => delete_todos(todos),
            3 => print_todos(todos),
            4 => update_todos(todos),
            5 => break,
            _ => println!("Something went wrong. Try again"),
        }
    }
}

fn main() {
    let mut todos: Vec<Todo> = vec![];
    todos_cli(&mut todos);
    // dbg!(random_id());
}
