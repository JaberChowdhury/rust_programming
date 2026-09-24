mod commands;
mod input;
mod utils;

use rusqlite::{Connection, Result};
use std::env;

use crate::commands::{add::add, del::del, search::search, show::show, up::up, show_flags::show_flags};
use crate::utils::init_db;

fn main() -> Result<()> {
    let conn = Connection::open("todos.db")?;
    init_db(&conn)?;

    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("add") => {
            add(&conn, &args)?;
        }
        Some("del") | Some("delete") => {
            del(&conn, &args)?;
        }
        Some("show") | Some("list") => {
            show(&conn)?;
        }
        Some("up") | Some("update") => {
            up(&conn, &args)?;
        }
        Some("search") => {
            search(&conn, &args)?;
        }
        Some("--help") | Some("-h") => {
            show_flags();
        }
        _ => {
            println!("Unknown or missing command");
            show_flags();
        }
    }
    Ok(())
}
