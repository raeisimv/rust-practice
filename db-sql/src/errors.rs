use std::error::Error;

pub type DbError = Box<dyn Error>;
pub type DbResult<T = (), E = DbError> = Result<T, E>;
