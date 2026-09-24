mod input;

use input::take_input;
mod utils;
use utils::init_db;

use rusqlite::{Connection, Result};

use crate::utils::{
    Todo, add_todo, clear_terminal, delete_todo, select_all_todos, toggle_todo, update_todo,
};

fn print_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("\n  (No tasks found)\n");
        return;
    }
    println!("\n=================== TODOS ===================");
    for t in todos {
        let mark = if t.is_completed { "[X]" } else { "[ ]" };
        let desc = match &t.description {
            Some(d) if !d.is_empty() => format!(" - {d}"),
            _ => String::new(),
        };
        println!("{mark} #{:<3} {}{}", t.id, t.title, desc);
    }
    println!("=============================================\n");
}

fn main() -> Result<()> {
    let conn = Connection::open("todos.db")?;
    init_db(&conn)?;

    println!("Welcome to Todo CLI!");

    loop {
        println!("\n[1] List Todos");
        println!("[2] Add Todo");
        println!("[3] Toggle Done / Pending");
        println!("[4] Update Todo Details");
        println!("[5] Delete Todo");
        println!("[6] Exit");

        let choice: u32 = take_input("Choose an option [1-6]:");

        match choice {
            1 => {
                let todos = select_all_todos(&conn)?;
                clear_terminal();
                print_todos(&todos);
            }
            2 => {
                let title: String = take_input("Title:");
                let desc_input: String = take_input("Description (press Enter to skip):");
                let desc = if desc_input.is_empty() {
                    None
                } else {
                    Some(desc_input.as_str())
                };

                let new_id = add_todo(&conn, &title, desc)?;
                println!("Added todo with ID: #{new_id}");
            }
            3 => {
                let id: i64 = take_input("Todo ID to toggle:");
                let is_done: bool = take_input("Mark as completed? (true / false):");

                match toggle_todo(&conn, id, is_done)? {
                    true => println!("Todo #{id} updated."),
                    false => println!("Error: Todo #{id} not found."),
                }
            }
            4 => {
                let id: i64 = take_input("Todo ID to update:");
                let title: String = take_input("New title:");
                let desc_input: String = take_input("New description (press Enter to skip):");
                let is_completed: bool = take_input("Is completed? (true / false):");

                let todo = Todo {
                    id,
                    title,
                    description: if desc_input.is_empty() {
                        None
                    } else {
                        Some(desc_input)
                    },
                    is_completed,
                };

                let affected = update_todo(&conn, &todo)?;
                if affected > 0 {
                    println!("Todo #{id} updated successfully.");
                } else {
                    println!("Error: Todo #{id} not found.");
                }
            }
            5 => {
                let id: i64 = take_input("Todo ID to delete:");
                let affected = delete_todo(&conn, id)?;
                if affected > 0 {
                    println!("Todo #{id} deleted.");
                } else {
                    println!("Error: Todo #{id} not found.");
                }
            }
            6 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please choose between 1 and 6."),
        }
    }

    Ok(())
}
