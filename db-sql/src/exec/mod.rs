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
                let Some(tbl) = self.tables.get(table) else {
                    return Err(ExecutionError::TableNotFound);
                };
            }
            SqlStatement::Insert { table, values } => {
                let Some(tbl) = self.tables.get(table) else {
                    return Err(ExecutionError::TableNotFound);
                };
            }
            SqlStatement::Create { table, .. } => {
                if self.tables.get(table).is_some() {
                    return Err(ExecutionError::TableAlreadyExists);
                };
            }
            _ => {}
        }
        Ok(())
    }
}
