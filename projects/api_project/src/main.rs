mod todo;
use crate::todo::{CreateTodo, Todo, UpdateTodo};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use std::sync::{Arc, Mutex};

struct AppState {
    todos: Mutex<Vec<Todo>>,
    next_id: Mutex<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            todos: Mutex::new(vec![
                Todo {
                    id: 1,
                    title: "Learn Rust".to_string(),
                    done: false,
                },
                Todo {
                    id: 2,
                    title: "Build an API".to_string(),
                    done: true,
                },
            ]),
            next_id: Mutex::new(3),
        }
    }
}
type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(get_todos).post(create_todo))
        .route(
            "/todos/:id",
            axum::routing::put(update_todo).delete(delete_todo),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server started on port 8080");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "hello world"
}

async fn get_todos(State(state): State<SharedState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap().clone();
    Json(todos)
}

async fn create_todo(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = state.todos.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();

    let todo = Todo {
        id: *next_id,
        title: payload.title,
        done: false,
    };

    *next_id += 1;
    todos.push(todo.clone());

    (StatusCode::CREATED, Json(todo))
}

async fn update_todo(
    Path(id): Path<usize>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = state.todos.lock().unwrap();

    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if let Some(title) = payload.title {
            todo.title = title;
        }
        if let Some(done) = payload.done {
            todo.done = done;
        }
        Ok(Json(todo.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_todo(Path(id): Path<usize>, State(state): State<SharedState>) -> StatusCode {
    let mut todos = state.todos.lock().unwrap();
    let len_before = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() != len_before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
