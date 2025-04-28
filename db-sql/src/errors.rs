use std::error::Error;

pub type DbError = Box<dyn Error>;
pub type DbResult<T = (), E = DbError> = Result<T, E>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionError {
    TableNotFound,
    TableAlreadyExists,
}
