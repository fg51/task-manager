use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::domain::repository::CreateTodo;
use crate::domain::repository::TodoRepository;
use crate::domain::repository::UpdateTodo;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_todo(
    State(repository): State<Arc<impl TodoRepository>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo(
    Path(id): Path<i32>,
    State(repository): State<Arc<impl TodoRepository>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo(State(repository): State<Arc<impl TodoRepository>>) -> impl IntoResponse {
    let todo = repository.all();
    (StatusCode::OK, Json(todo))
}

pub async fn update_todo(
    Path(id): Path<i32>,
    State(repository): State<Arc<impl TodoRepository>>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo(
    Path(id): Path<i32>,
    State(repository): State<Arc<impl TodoRepository>>,
) -> StatusCode {
    repository
        .delete(id)
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
