use crate::Todo;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Row, Sqlite, SqlitePool, Transaction,
};
use std::str::FromStr;

type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

/// SQLiteのコネクションプールを作成する
pub(crate) async fn create_sqlite_pool(database_url: &str) -> DbResult<SqlitePool> {
    let conn = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(conn)
        .await?;

    Ok(sqlite_pool)
}

/// マイグレーションの実行
pub(crate) async fn migrate_database(pool: &SqlitePool) -> DbResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

/// Todoを取得する
pub (crate) async fn get_todos(pool: &SqlitePool) -> DbResult<Vec<Todo>> {
    let todos: Vec<Todo> = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(pool)
        .await?;
    Ok(todos)
}

/// Todoを追加する
pub (crate) async fn insert_todo(pool: &SqlitePool, description: String) -> DbResult<Todo> {
    let todo: Todo = sqlx::query_as::<_, Todo>("INSERT INTO todos (description, completed) VALUES (?, ?) RETURNING *")
        .bind(description)
        .bind(false)
        .fetch_one(pool)
        .await?;
    Ok(todo)
}
