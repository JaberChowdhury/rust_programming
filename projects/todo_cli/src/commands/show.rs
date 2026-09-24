use crate::utils::select_all_todos;
use rusqlite::{Connection, Result};

pub fn show(conn: &Connection) -> Result<()> {
    let todos = select_all_todos(conn)?;
    if todos.is_empty() {
        println!("No todos found.");
        return Ok(());
    }

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
