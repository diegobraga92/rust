use axum::{
    routing::{get, post},
    Router,
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};


type SharedTodoList = Arc<Mutex<Vec<Todo>>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    let todos: SharedTodoList = Arc::new(Mutex::new(Vec::new()));
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check))
        .route("/todos", post(add_todo))
        .with_state(todos);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

#[derive(Debug, Deserialize)]
struct NewTodo {
    title: String,
}

async fn add_todo(
    State(todos): State<SharedTodoList>,
    Json(payload): Json<NewTodo>,
) -> Json<Todo> {
    let mut todos = todos.lock().unwrap();
    let id = todos.len() + 1;
    let todo = Todo {
        id,
        title: payload.title,
        completed: false,
    };
    todos.push(todo.clone());
    Json(todo)
}