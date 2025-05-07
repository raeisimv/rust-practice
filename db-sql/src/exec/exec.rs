use crate::{
    errors::{DbResult, ExecutionError},
    exec::{Row, Table},
    parser::{Identifier, SqlStatement},
};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ExecutionResult<'a> {
    Select(Vec<Row<'a>>),
    Insert,
    Create,
    Delete(usize),
}

impl Display for ExecutionResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let ExecutionResult::Select(rows) = self {
            for (i, row) in rows.iter().enumerate() {
                if i == 0 {
                    for col in row.columns.iter() {
                        write!(f, "{} \t| ", col.name)?;
                    }
                }
                write!(f, "\n")?;
                for val in row.values.iter() {
                    write!(f, "{} \t| ", val.1)?;
                }
            }
        } else {
            write!(f, "{self:?}")?;
        }

        Ok(())
    }
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
    pub fn run(&mut self, input: &str) -> DbResult<ExecutionResult, ExecutionError> {
        let stmt = SqlStatement::try_from(input);
        if let Err(e) = stmt {
            eprintln!("PARSER ERR: {:?}", e);
            return Err(ExecutionError::ParserError(e.to_string()));
        }
        let res = self.exec(&stmt.unwrap())?;
        println!("{}", res);
        Ok(res)
    }

    pub fn exec(&mut self, cmd: &SqlStatement) -> DbResult<ExecutionResult, ExecutionError> {
        match cmd {
            SqlStatement::Select { table, .. } => {
                let Some(tbl) = self.tables.get(table) else {
                    return Err(ExecutionError::TableNotFound);
                };

                Ok(ExecutionResult::Select(tbl.iter().collect()))
            }
            SqlStatement::Insert { table, values } => {
                let Some(tbl) = self.tables.get_mut(table) else {
                    return Err(ExecutionError::TableNotFound);
                };

                tbl.insert(values.iter().map(|x| x.to_string()).collect());
                Ok(ExecutionResult::Insert)
            }
            SqlStatement::Create { table, columns } => {
                if self.tables.get(table).is_some() {
                    return Err(ExecutionError::TableAlreadyExists);
                };

                let t = Table::new(columns.clone());
                self.tables.insert(table.clone(), t);

                Ok(ExecutionResult::Create)
            }
            SqlStatement::Delete { table, condition } => {
                let Some(tbl) = self.tables.get_mut(table) else {
                    return Err(ExecutionError::TableNotFound);
                };
                if condition.is_none() {
                    tbl.clear_all();
                } else {
                    tbl.delete(0);
                }
                Ok(ExecutionResult::Delete(0))
            }
        }
    }
}
