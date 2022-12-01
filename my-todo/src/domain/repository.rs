use serde::{Deserialize, Serialize};

use crate::errors::Result;

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo>;
    fn delete(&self, id: i32) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }

    pub const fn new_with_completed(id: i32, text: String, completed: bool) -> Self {
        Self {
            id,
            text,
            completed,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

impl CreateTodo {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl UpdateTodo {
    pub fn new(text: Option<String>, completed: Option<bool>) -> Self {
        Self { text, completed }
    }

    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn completed(&self) -> Option<bool> {
        self.completed.clone()
    }
}
