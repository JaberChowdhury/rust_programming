use crate::utils::{get_todo_by_id, update_todo};
use rusqlite::{Connection, Result};
use std::str::FromStr;

pub fn up(conn: &Connection, args: &[String]) -> Result<()> {
    if args.len() < 3 {
        println!("Error: Missing todo ID to update.");
        return Ok(());
    }

    let id = match i64::from_str(&args[2]) {
        Ok(val) => val,
        Err(_) => {
            println!("Error: Invalid todo ID.");
            return Ok(());
        }
    };

    let mut todo = match get_todo_by_id(conn, id)? {
        Some(t) => t,
        None => {
            println!("Todo #{id} not found.");
            return Ok(());
        }
    };

    let mut updated = false;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                if i + 1 < args.len() {
                    todo.title = args[i + 1].clone();
                    updated = true;
                    i += 1;
                }
            }
            "--desc" => {
                if i + 1 < args.len() {
                    todo.description = Some(args[i + 1].clone());
                    updated = true;
                    i += 1;
                }
            }
            "--done" => {
                if i + 1 < args.len() {
                    let d = &args[i + 1];
                    if d == "1" || d.to_lowercase() == "true" {
                        todo.is_completed = true;
                        updated = true;
                    } else if d == "0" || d.to_lowercase() == "false" {
                        todo.is_completed = false;
                        updated = true;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    if updated {
        update_todo(conn, &todo)?;
        println!("Updated todo #{id}");
    } else {
        println!("No update options provided. Example: todo_cli up {id} --done 1");
    }

    Ok(())
}
