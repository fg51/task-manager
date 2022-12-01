use thiserror::Error;

pub type Result<T> = std::result::Result<T, ErrorKind>;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("{0}")]
    RepositoryError(RepositoryError),
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}
