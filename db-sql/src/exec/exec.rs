use crate::errors::{DbResult, ExecutionError};
use crate::exec::Table;
use crate::parser::{Identifier, SqlStatement};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionResult {
    Select,
    Insert,
    Update,
    Delete,
    Create,
}

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
    pub fn exec(&mut self, cmd: &SqlStatement) -> DbResult<ExecutionResult, ExecutionError> {
        match cmd {
            SqlStatement::Select { table, .. } => {
                let Some(tbl) = self.tables.get(table) else {
                    return Err(ExecutionError::TableNotFound);
                };

                Ok(ExecutionResult::Select)
            }
            SqlStatement::Insert { table, values } => {
                let Some(tbl) = self.tables.get(table) else {
                    return Err(ExecutionError::TableNotFound);
                };

                Ok(ExecutionResult::Insert)
            }
            SqlStatement::Create { table, .. } => {
                if self.tables.get(table).is_some() {
                    return Err(ExecutionError::TableAlreadyExists);
                };

                Ok(ExecutionResult::Create)
            }
            SqlStatement::Delete { table, .. } => {
                let Some(tbl) = self.tables.get(table) else {
                    return Err(ExecutionError::TableNotFound);
                };

                Ok(ExecutionResult::Delete)
            }
        }
    }
}
