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
async fn handle_add_todo(sqlite_pool: SqliteState<'_>, description: String) -> Result<Todo, String> {
    let todo = database::insert_todo(&sqlite_pool, description)
        .await
        .map_err(|e| e.to_string())?;
    Ok(todo)
}

#[tauri::command]
async fn handle_update_todo(sqlite_pool: SqliteState<'_>, id: i64, completed: bool) -> Result<Todo, String> {
    let todo = database::update_todo(&sqlite_pool, id, completed)
        .await
        .map_err(|e| e.to_string())?;
    Ok(todo)
}

#[tauri::command]
async fn handle_remove_todo(sqlite_pool: SqliteState<'_>, id: i64) -> Result<(), String> {
    database::delete_todo(&sqlite_pool, id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    use tauri::async_runtime::block_on;

    let config_dir = directories::ProjectDirs::from("work", "eqseqs", "todoapp")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .unwrap_or_else(|| panic!("Failed to get config directory"));
    let db_filename = "database.sqlite";
    let db_filepath = config_dir.join(db_filename);

    // データベースのディレクトリがないなら作成
    if !config_dir.exists() {
        println!("Creating directory to {}", config_dir.display());
        std::fs::create_dir_all(&config_dir)?;
    }

    // データベースファイルを開く前に有無を確認する
    let db_exists = &db_filepath.exists();

    // データベースファイルを開く
    let db_url = format!(
        "sqlite://{}",
        db_filepath.clone().into_os_string().into_string().unwrap()
    );
    let sqlite_pool = block_on(database::create_sqlite_pool(&db_url))?;

    // データベースファイルが無かったならマイグレーションを実行
    if !db_exists {
        println!("Executing migrations ...");
        block_on(database::migrate_database(&sqlite_pool))?;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handle_get_todolist,
            handle_add_todo,
            handle_update_todo,
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
