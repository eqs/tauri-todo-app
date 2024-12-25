use tauri::{Manager, State};
use serde::{Deserialize, Serialize};

pub(crate) mod database;

type SqliteState<'a> = State<'a, sqlx::SqlitePool>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    todos: Vec<Todo>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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
async fn handle_get_todolist(sqlite_pool: SqliteState<'_>) -> Result<TodoList, String> {
    let todos = database::get_todos(&sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;
    let todolist = TodoList { todos };
    Ok(todolist)
}

#[tauri::command]
async fn handle_add_todo(sqlite_pool: SqliteState<'_>, description: String) -> Result<(), String> {
    println!("handle_add_todo ---------------");
    database::insert_todo(&sqlite_pool, description)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn handle_remove_todo(sqlite_pool: SqliteState<'_>, id: i64) -> Result<(), String> {
    println!("handle_remove_todo ---------------");
    dbg!(id);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    use tauri::async_runtime::block_on;

    let db_filepath = "./database.sqlite";
    let db_url = format!("sqlite://{}", db_filepath);
    let sqlite_pool = block_on(database::create_sqlite_pool(&db_url))?;

    if std::fs::metadata(&db_filepath).is_ok() {
        block_on(database::migrate_database(&sqlite_pool))?;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handle_get_todolist,
            handle_add_todo,
            handle_remove_todo,
        ])
        // ハンドラからコネクションプールにアクセスできるようにする
        .setup(|app| {
            app.manage(sqlite_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
