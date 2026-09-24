use crate::utils::delete_todo;
use rusqlite::{Connection, Result};
use std::str::FromStr;

pub fn del(conn: &Connection, args: &[String]) -> Result<()> {
    if args.len() < 3 {
        println!("Error: Missing todo ID to delete.");
        return Ok(());
    }

    if let Ok(id) = i64::from_str(&args[2]) {
        let deleted = delete_todo(conn, id)?;
        if deleted > 0 {
            println!("Deleted todo #{id}");
        } else {
            println!("Todo #{id} not found.");
        }
    } else {
        println!("Error: Invalid todo ID.");
    }

    Ok(())
}
