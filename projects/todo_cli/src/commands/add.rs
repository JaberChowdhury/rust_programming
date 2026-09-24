use crate::input::take_input;
use crate::utils::add_todo;
use rusqlite::{Connection, Result};

pub fn add(conn: &Connection, args: &[String]) -> Result<()> {
    if args.len() == 2 {
        let title: String = take_input("Title:");
        let desc_input: String = take_input("Description (press Enter to skip):");
        let desc = if desc_input.trim().is_empty() {
            None
        } else {
            Some(desc_input.trim())
        };

        let new_id = add_todo(conn, &title, desc)?;
        println!("Added todo with ID: #{new_id}");
    } else {
        // Parse basic args: --title "X" --desc "Y"
        let mut title = String::new();
        let mut desc = None;

        let mut i = 2;
        while i < args.len() {
            match args[i].as_str() {
                "--title" => {
                    if i + 1 < args.len() {
                        title = args[i + 1].clone();
                        i += 1;
                    }
                }
                "--desc" => {
                    if i + 1 < args.len() {
                        desc = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if title.is_empty() {
            println!("Error: --title is required when using flags");
            return Ok(());
        }

        let new_id = add_todo(conn, &title, desc.as_deref())?;
        println!("Added todo with ID: #{new_id}");
    }
    Ok(())
}
