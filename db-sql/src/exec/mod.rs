mod table;

use crate::{
    errors::{DbResult, ExecutionError},
    parser::{Identifier, SqlStatement},
};
use std::collections::HashMap;
pub use table::*;

#[derive(Debug)]
pub struct ExecutionContext {
    tables: HashMap<Identifier, Table>,
}
impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }
    pub fn exec(&mut self, cmd: &SqlStatement) -> DbResult<(), ExecutionError> {
        match cmd {
            SqlStatement::Select { table, .. } => {
                if self.tables.get(table).is_none() {
                    return Err(ExecutionError::TableNotFound);
                }
            }
            _ => {}
        }
        Ok(())
    }
}
