use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::errors::Result;
use crate::errors::{ErrorKind, RepositoryError};

use crate::domain::repository::TodoRepository;
use crate::domain::repository::{CreateTodo, Todo, UpdateTodo};

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
        self.store.read().unwrap()
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text());
        store.insert(id, todo.clone());
        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.read_store_ref();
        store.get(&id).map(|todo| todo.clone())
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        Vec::from_iter(store.values().map(|todo| todo.clone()))
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo> {
        let mut store = self.write_store_ref();

        let (text, completed) = {
            let todo = store.get(&id).ok_or(RepositoryNotFound::new(id))?;
            let text = payload.text().unwrap_or(todo.text());
            let completed = payload.completed().unwrap_or(todo.completed());
            (text, completed)
        };

        let todo = Todo::new_with_completed(id, text, completed);
        store.insert(id, todo.clone());
        Ok(todo)
    }

    fn delete(&self, id: i32) -> Result<()> {
        let mut store = self.write_store_ref();
        match store.remove(&id) {
            Some(_) => Ok(()),
            None => Err(RepositoryNotFound::new(id)),
        }
    }
}

struct RepositoryNotFound;

impl RepositoryNotFound {
    fn new(id: i32) -> ErrorKind {
        ErrorKind::RepositoryError(RepositoryError::NotFound(id))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo_crud_scenario() {
        let text = "todo text".to_string();
        let id = 1;
        let expected = Todo::new(id, text.clone());

        // create
        let repository = TodoRepositoryForMemory::new();
        let todo = repository.create(CreateTodo::new(text));
        assert_eq!(expected, todo);

        // find
        let todo = repository.find(todo.id()).unwrap();
        assert_eq!(expected, todo);

        // all
        let todos = repository.all();
        assert_eq!(vec![expected], todos);

        // update
        let text = "update todo text".to_string();
        let todo = repository
            .update(1, UpdateTodo::new(Some(text.clone()), Some(true)))
            .expect("failed update todo.");
        assert_eq!(id, todo.id());
        assert_eq!(text, todo.text());
        assert_eq!(true, todo.completed());

        // delete
        let res = repository.delete(id);
        assert!(res.is_ok())
    }
}
