import { ChangeEventHandler, FC, useEffect, useState } from "react";
import type { Todo } from "../types/todo";
import {
  Box,
  Button,
  Card,
  Checkbox,
  Grid,
  Modal,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { modalInnerStyle } from "../styles/modal";

type Props = {
  todo: Todo;
  onUpdate: (todo: Todo) => void;
  onDelete: (id: number) => void;
};

const TodoItem: FC<Props> = ({ todo, onUpdate, onDelete }) => {
  const [editing, setEditing] = useState(false);
  const [editText, setEditText] = useState("");

  useEffect(() => {
    setEditText(todo.text);
  }, [todo]);

  const handleCompletedCheckbox: ChangeEventHandler = (e) => {
    onUpdate({
      ...todo,
      completed: !todo.completed,
    });
  };

  const onCloseEditModal = () => {
    onUpdate({
      ...todo,
      text: editText,
    });
    setEditing(false);
  };

  const handleDelete = () => onDelete(todo.id);

  return (
    <Card sx={{ p: 1 }}>
      <Grid container spacing={2} alignItems="center">
        <Grid item xs={1}>
          <Checkbox
            onChange={handleCompletedCheckbox}
            checked={todo.completed}
          />
        </Grid>
        <Grid item xs={9}>
          <Stack spacing={1}>
            <Typography variant="caption" fontsize={16}>
              {todo.text}
            </Typography>
          </Stack>
        </Grid>
        <Grid item xs={2}>
          {
            /*
          <Button onClick={handleDelete} color="error">
            delete
          </Button>
          */
          }
          <Stack direction="row" spacing={1}>
            <Button onClick={() => setEditing(true)} color="info">edit</Button>
            <Button onClick={handleDelete} color="error">
              delete
            </Button>
          </Stack>
        </Grid>
      </Grid>
      <Modal open={editing} onClose={onCloseEditModal}>
        <Box sx={modalInnerStyle}>
          <Stack spacing={2}>
            <TextField
              size="small"
              label="todo text"
              defaultValue={todo.text}
              onChange={(e) => setEditText(e.target.value)}
            />
          </Stack>
        </Box>
      </Modal>
    </Card>
  );
};

export default TodoItem;
