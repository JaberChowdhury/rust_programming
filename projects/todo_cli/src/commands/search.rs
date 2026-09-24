use crate::utils::search_todos;
use rusqlite::{Connection, Result};

pub fn search(conn: &Connection, args: &[String]) -> Result<()> {
    if args.len() < 3 {
        println!("Error: Missing search query.");
        return Ok(());
    }

    let query = &args[2];
    let todos = search_todos(conn, query)?;

    if todos.is_empty() {
        println!("No todos found matching: {}", query);
        return Ok(());
    }

    println!("Search results for '{}':", query);
    println!("{:-<60}", "");
    println!("{:<5} | {:<30} | {:<10}", "ID", "Title", "Status");
    println!("{:-<60}", "");

    for todo in todos {
        let status = if todo.is_completed { "Done" } else { "Pending" };
        println!("{:<5} | {:<30} | {:<10}", todo.id, todo.title, status);
        if let Some(desc) = todo.description {
            if !desc.is_empty() {
                println!("      > {}", desc);
            }
        }
    }
    println!("{:-<60}", "");
    Ok(())
}
