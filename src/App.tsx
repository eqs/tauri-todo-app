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
import DeleteIcon from '@mui/icons-material/Delete';
import "./App.css";

import {
  Todo,
} from "./types";
import {
  handleGetTodoList,
  handleAddTodo,
  handleRemoveTodo,
} from "./api";

function TodoListComponent() {
  const [todos, setTodos] = useState<Todo[]>([]);

  useEffect(() => {
    handleGetTodoList()
      .then((todoList) => {
        setTodos(todoList.todos);
      });
  }, []);

  const handleOnClick = () => {
    handleAddTodo("test")
      .then((newTodo) => {
        let newTodos = [...todos, newTodo];
        console.log(newTodo);
        setTodos(newTodos);
      });
  };

  return (
    <>
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell style={{ width: 16 }}>ID</TableCell>
              <TableCell>Description</TableCell>
              <TableCell style={{ width: 16 }}>Completed</TableCell>
              <TableCell style={{ width: 16 }}></TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {todos.map((todo: Todo) => {
              const onDeleteClicked = () => {
                handleRemoveTodo(todo.id);
              };

              return (
                <TableRow key={todo.id}>
                  <TableCell>{todo.id}</TableCell>
                  <TableCell>{todo.description}</TableCell>
                  <TableCell>
                    <Checkbox checked={todo.completed} />
                  </TableCell>
                  <TableCell>
                    <Button onClick={onDeleteClicked}><DeleteIcon /></Button>
                  </TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </TableContainer>
      <Button onClick={handleOnClick}>+</Button>
    </>
  );
}

function App() {
  return (
    <main className="container">
      <h1>It works!</h1>
      <TodoListComponent />
    </main>
  );
}

export default App;
