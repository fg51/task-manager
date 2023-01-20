import { FC } from "react";
import { Stack, Typography } from "@mui/material";

import type { Todo } from "../types/todo";
import TodoItem from "./TodoItem";

type Props = {
  todos: Todo[];
  onUpdate: (todo: Todo) => void;
  onDelete: (id: number) => void;
};

const TodoList: FC<Props> = ({ todos, onUpdate, onDelete }) => {
  //const handleCompletedCheckbox = (todo: Todo) => {
  //  onUpdate({
  //    ...todo,
  //    completed: !todo.completed,
  //  });
  //};

  return (
    <Stack spacing={2}>
      <Typography variant="h2">todo list</Typography>
      <Stack spacing={2}>
        {
          /*
        {todos.map((todo) => (
          <Card key={todo.id} sx={{ p: 2 }}>
            <Stack direction="row" alignItems="center">
              <Checkbox
                checked={todo.completed}
                onChange={() => handleCompletedCheckbox(todo)}
              />
              <Typography variant="body1">{todo.text}</Typography>
            </Stack>
          </Card>
        ))}
        */
        }
        {todos.map((todo) => (
          <TodoItem
            key={todo.id}
            todo={todo}
            onUpdate={onUpdate}
            onDelete={onDelete}
          />
        ))}
      </Stack>
    </Stack>
  );
};

export default TodoList;
