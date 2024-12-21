import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Todo {
  id: number;
  description: string;
  completed: boolean;
}

const defaultTodos: Todo[] = [
  {
    id: 0,
    description: "買い物",
    completed: false,
  },
  {
    id: 1,
    description: "勉強",
    completed: true,
  },
  {
    id: 2,
    description: "睡眠",
    completed: true,
  },
  {
    id: 3,
    description: "ゲーム",
    completed: false,
  },
];

function App() {
  const [todos, setTodos] = useState<Todo[]>(defaultTodos);

  return (
    <main className="container">
      <h1>It works!</h1>
      {todos.map((todo, k) => {
        return (
          <div className="row">
            {todo.id}: {todo.description}, {todo.completed ? "done" : "in progress"}
          </div>
        );
      })}
    </main>
  );
}

export default App;
