import React, { useState, useEffect } from "react";
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

import DialogTitle from '@mui/material/DialogTitle';
import DialogContent from '@mui/material/DialogContent';
import Dialog from '@mui/material/Dialog';
import TextField from '@mui/material/TextField';

import "./App.css";

import {
  Todo,
} from "./types";
import {
  handleGetTodoList,
  handleAddTodo,
  handleUpdateTodo,
  handleRemoveTodo,
} from "./api";

export interface TodoInputDialogProps {
  open: boolean;
  value: string;
  onClose: (value: string) => void;
}

function TodoInputDialog(props: TodoInputDialogProps) {
  const { open, value, onClose } = props;

  const onKeyDown = (e: any) => {
    if (e.key === "Enter") {
      onClose(e.target.value);
    }
  };

  const handleOnClose = () => {
    onClose("");
  };

  return (
    <Dialog onClose={handleOnClose} open={open}>
      <DialogTitle>Add new Todo</DialogTitle>
      <DialogContent>
        <TextField
          id="outlined-basic"
          label="Description"
          variant="outlined"
          style={{ margin: "0.5em" }}
          defaultValue={value}
          onKeyDown={onKeyDown}
        />
      </DialogContent>
    </Dialog>
  );
}

function TodoListComponent() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [dialogOpen, setDialogOpen] = useState<boolean>(false);

  useEffect(() => {
    handleGetTodoList()
      .then((todoList) => {
        setTodos(todoList.todos);
      });
  }, []);

  const handleDialogOpen = () => {
    setDialogOpen(true);
  };

  const handleDialogClose = (value: string) => {
    setDialogOpen(false);
    if (value !== "") {
      handleAddTodo(value)
        .then((newTodo) => {
          let newTodos = [...todos, newTodo];
          setTodos(newTodos);
          setDialogOpen(false);
        });
    }
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
                handleRemoveTodo(todo.id)
                  .then(() => {
                    let newTodos =  todos.filter(t => t.id !== todo.id);
                    setTodos(newTodos);
                  });
              };

              const onCheckboxChanged = (e: React.ChangeEvent<HTMLInputElement>) => {
                handleUpdateTodo(todo.id, e.target.checked)
                  .then((updatedTodo: Todo) => {
                    let newTodos =  todos.map((t: Todo) => {
                      if (t.id == updatedTodo.id) {
                          return { ...t, completed: updatedTodo.completed };
                      } else {
                          return t
                      }
                    });
                    setTodos(newTodos);
                  });
              };

              return (
                <TableRow key={todo.id}>
                  <TableCell>{todo.id}</TableCell>
                  <TableCell>{todo.description}</TableCell>
                  <TableCell>
                    <Checkbox checked={todo.completed} onChange={onCheckboxChanged} />
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
      <Button onClick={handleDialogOpen}>+</Button>
      <TodoInputDialog
        open={dialogOpen}
        onClose={handleDialogClose}
        value={""}
      />
    </>
  );
}

function App() {
  return (
    <main className="container">
      <h1>Todo App</h1>
      <TodoListComponent />
    </main>
  );
}

export default App;
