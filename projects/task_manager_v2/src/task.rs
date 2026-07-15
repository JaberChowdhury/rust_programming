use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Completed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let task = Task {
            id,
            description,
            status: TaskStatus::Pending,
        };
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }
        self.tasks.iter().for_each(|task| {
            println!("[{}] [{}] {}", task.id, task.status, task.description);
        });
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Completed;
            Ok(())
        } else {
            Err(format!("Task with ID {id} not found."))
        }
    }

    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(format!("Task with ID {id} not found."))
        }
    }

    pub fn save_to_disk<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn load_from_disk<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::new());
        }
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Self::new()),
            Err(e) => return Err(e.into()),
        };
        let manager: Self = serde_json::from_reader(file)?;
        Ok(manager)
    }
}
