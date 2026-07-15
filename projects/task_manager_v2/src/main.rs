mod task;

use clap::{Parser, Subcommand};
use std::{path::StripPrefixError, process};
use task::TaskManager;

const DATA_FILE: &str = "tasks.json";

#[derive(Parser, Debug)]
#[command(name = "rust_task_manager")]
#[command(version, about = "A command-line task manager written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task with a description
    Add {
        /// The description of the task
        description: String,
    },
    /// Mark a task as completed by ID
    Complete {
        /// The ID of the task to complete
        id: u32,
    },
    /// List all tasks
    List,
    /// Delete a task by ID
    Delete {
        /// The ID of the task to delete
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut manager = match TaskManager::load_from_disk(DATA_FILE) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Error loading {DATA_FILE}: {err}");
            process::exit(1);
        }
    };

    let mut modified = false;

    match cli.command {
        Commands::Add { description } => {
            manager.add_task(description);
            println!("Task added successfully.");
            modified = true;
        }
        Commands::Complete { id } => match manager.complete_task(id) {
            Ok(()) => {
                println!("Task {id} marked as completed.");
                modified = true;
            }
            Err(err) => {
                eprintln!("Error: {err}");
                process::exit(1);
            }
        },
        Commands::List => {
            manager.list_tasks();
        }
        Commands::Delete { id } => match manager.delete_task(id) {
            Ok(()) => {
                println!("Task {id} deleted successfully.");
                modified = true;
            }
            Err(err) => {
                eprintln!("Error: {err}");
                process::exit(1);
            }
        },
    }

    if modified && let Err(err) = manager.save_to_disk(DATA_FILE) {
        eprintln!("Error saving tasks to {DATA_FILE}: {err}");
        process::exit(1);
    }
}
