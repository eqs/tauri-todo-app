import { useState, useEffect } from "react";
import Paper from "@mui/material/Paper";
import TableContainer from "@mui/material/TableContainer";
import Table from "@mui/material/Table";
import TableHead from "@mui/material/TableHead";
import TableBody from "@mui/material/TableBody";
import TableRow from "@mui/material/TableRow";
import TableCell from "@mui/material/TableCell";
import Checkbox from "@mui/material/Checkbox";
import Button from "@mui/material/Button";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Todo {
  id: number;
  description: string;
  completed: boolean;
}

function TodoList() {
  const [todos, setTodos] = useState<Todo[]>([]);

  useEffect(() => {
    async function fetchData() {
      let todoList: any = await invoke("handle_get_todolist");
      setTodos(todoList.todos);
    }
    fetchData();
  }, []);

  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>ID</TableCell>
            <TableCell>Description</TableCell>
            <TableCell>Completed</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {todos.map((todo: Todo) => {
            return (
              <TableRow>
                <TableCell>{todo.id}</TableCell>
                <TableCell>{todo.description}</TableCell>
                <TableCell>
                  <Checkbox checked={todo.completed} />
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </TableContainer>
  );
}

function App() {
  return (
    <main className="container">
      <h1>It works!</h1>
      <TodoList />
      <Button>+</Button>
    </main>
  );
}

export default App;
