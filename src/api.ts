import { invoke } from "@tauri-apps/api/core";
import { Todo, TodoList } from "./types";

export async function handleGetTodoList(): Promise<TodoList> {
  return await invoke<TodoList>("handle_get_todolist");
}

export async function handleAddTodo(description: string): Promise<Todo> {
  return await invoke<Todo>("handle_add_todo", { description });
}

export async function handleRemoveTodo(id: number): Promise<void> {
  return await invoke<void>("handle_remove_todo", { id });
}
