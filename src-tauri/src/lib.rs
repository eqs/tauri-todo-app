use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    todos: Vec<Todo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: i64,
    description: String,
    completed: bool,
}

impl Todo {
    fn new(id: i64, description: String, completed: bool) -> Self {
        Self { id, description, completed }
    }
}

#[tauri::command]
fn handle_get_todolist() -> Result<TodoList, String> {
    let todolist = TodoList {
        todos: vec![
            Todo::new(0, "買い物".to_string(), false),
            Todo::new(1, "勉強".to_string(), true),
            Todo::new(2, "睡眠".to_string(), true),
            Todo::new(3, "ゲーム".to_string(), false),
            Todo::new(4, "食事".to_string(), true),
        ]
    };

    Ok(todolist)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle_get_todolist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
