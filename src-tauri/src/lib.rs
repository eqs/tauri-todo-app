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

#[tauri::command]
fn handle_add_todo(todo: Todo) -> Result<(), String> {
    println!("handle_add_todo ---------------");
    dbg!(&todo);
    Ok(())
}

#[tauri::command]
fn handle_remove_todo(id: i64) -> Result<(), String> {
    println!("handle_remove_todo ---------------");
    dbg!(id);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handle_get_todolist,
            handle_add_todo,
            handle_remove_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
