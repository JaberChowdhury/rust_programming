use std::io;

#[derive(Debug, Clone)]
struct Task {
    id: i32,
    title: String,
    description: String,
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut input = String::new();

    println!("Please enter the title:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let title_input = input.trim().to_string();

    input.clear();

    println!("Please enter the description:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let desc_input = input.trim().to_string();

    tasks.push(Task {
        id: tasks.len() as i32 + 1,
        title: title_input,
        description: desc_input,
    });

    dbg!(tasks);
}
fn show_task(tasks: &mut Vec<Task>) {
    for task in tasks {
        dbg!(task);
    }
}
fn delete_task(tasks: &mut Vec<Task>, id: i32) {
    let mut temp: Vec<Task> = vec![];
    let mut is_deleted = false;
    for task in tasks.iter() {
        if task.id != id {
            temp.push(task.clone());
        } else {
            is_deleted = true;
        }
    }
    if is_deleted {
        *tasks = temp;
        println!("Task is deleted");
    } else {
        println!("Task didnt found.");
    }
}
fn update_task(tasks: &mut Vec<Task>) {
    let mut input = String::new();
    let mut is_found = false;

    println!("Please enter the task id to update:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let id: i32 = input.trim().parse::<i32>().expect("Invalid ID format");

    input.clear();

    println!("Please enter the title:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let title_input = input.trim().to_string();

    input.clear();

    println!("Please enter the description:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    let desc_input = input.trim().to_string();

    for task in tasks.iter_mut() {
        if task.id == id {
            task.title = title_input;
            task.description = desc_input;
            is_found = true;
            break;
        }
    }

    if !is_found {
        println!("The id is invalid");
    } else {
        println!("The task is updated");
    }
}
fn user_input(tasks: &mut Vec<Task>) {
    let mut choise = String::new();
    println!("Please select an option :: ");
    println!("1. Add task\n2. Show task\n3. Update task\n4. Delete");
    io::stdin()
        .read_line(&mut choise)
        .expect("Failed to take input");
    match choise.trim() {
        "1" => {
            println!("You selected Option 1 (Add Task)");
            add_task(tasks);
        }
        "2" => {
            println!("You selected Option 2 (Show Tasks)");
            show_task(tasks);
        }
        "3" => {
            println!("You selected Option 3 (Update Task)");
            update_task(tasks);
        }
        "4" => {
            for task in tasks.iter() {
                print!("id : {}\t", task.id);
            }
            println!("You selected Option 3 (Delete Task)");

            delete_task(tasks, 0);
        }
        _ => {
            println!("Invalid choice! Please select a valid option.");
        }
    }
}
fn main() {
    let mut tasks = vec![];

    user_input(&mut tasks);
}
