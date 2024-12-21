export interface TodoList {
  todos: Todo[];
}

export interface Todo {
  id: number;
  description: string;
  completed: boolean;
}
