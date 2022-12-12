use std::sync::Arc;

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Path, State},
    http::Request,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::de::DeserializeOwned;
use validator::Validate;

use crate::domain::repository::CreateTodo;
use crate::domain::repository::TodoRepository;
use crate::domain::repository::UpdateTodo;

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let message = format!("Json parse error: [{}]", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validattion error: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_todo(
    State(repository): State<Arc<impl TodoRepository>>,
    //Json(payload): Json<CreateTodo>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
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
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
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
