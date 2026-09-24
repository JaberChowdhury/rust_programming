use std::io::{self, Write};

use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            title       TEXT NOT NULL,
            description TEXT,
            is_done     INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

pub fn add_todo(conn: &Connection, title: &str, desc: Option<&str>) -> Result<i64> {
    conn.execute(
        "INSERT INTO todos (title, description) VALUES (?1, ?2)",
        params![title, desc],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn select_all_todos(conn: &Connection) -> Result<Vec<Todo>> {
    let mut stmt =
        conn.prepare("SELECT id, title, description, is_done FROM todos ORDER BY id ASC")?;

    let rows = stmt.query_map([], |row| {
        let is_done_num: i32 = row.get(3)?;
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?, // rusqlite converts NULL to None automatically
            is_completed: is_done_num != 0,
        })
    })?;

    // Collects rows and short-circuits on the first error automatically
    rows.collect()
}

pub fn delete_todo(conn: &Connection, todo_id: i64) -> Result<usize> {
    conn.execute("DELETE FROM todos WHERE id = ?1", params![todo_id])
}

pub fn update_todo(conn: &Connection, data: &Todo) -> Result<usize> {
    conn.execute(
        "UPDATE todos
         SET title = ?1, description = ?2, is_done = ?3
         WHERE id = ?4",
        params![
            data.title,
            data.description,
            data.is_completed as i32,
            data.id
        ],
    )
}

pub fn toggle_todo(conn: &Connection, id: i64, is_done: bool) -> Result<bool> {
    let rows_affected = conn.execute(
        "UPDATE todos SET is_done = ?1 WHERE id = ?2",
        params![is_done as i32, id],
    )?;

    Ok(rows_affected > 0)
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush().unwrap();
}

pub fn get_todo_by_id(conn: &Connection, id: i64) -> Result<Option<Todo>> {
    let mut stmt = conn.prepare("SELECT id, title, description, is_done FROM todos WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let is_done_num: i32 = row.get(3)?;
        Ok(Some(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            is_completed: is_done_num != 0,
        }))
    } else {
        Ok(None)
    }
}

pub fn search_todos(conn: &Connection, query: &str) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare("SELECT id, title, description, is_done FROM todos WHERE title LIKE ?1 OR description LIKE ?1 ORDER BY id ASC")?;
    let search_term = format!("%{}%", query);
    
    let rows = stmt.query_map(params![search_term], |row| {
        let is_done_num: i32 = row.get(3)?;
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            is_completed: is_done_num != 0,
        })
    })?;

    rows.collect()
}
