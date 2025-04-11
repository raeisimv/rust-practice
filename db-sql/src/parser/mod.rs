mod create;
mod select;

use nom::{bytes::complete::take_while1, combinator::map, IResult, Parser};
pub use select::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SqlStatement {
    Select {
        table: String,
        columns: Vec<String>,
        condition: Option<String>,
    },
    Create {
        table: String,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum SqlDataType {
    Integer,
    String,
}
#[derive(Clone, Debug)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: SqlDataType,
}

fn identifier(input: &str) -> IResult<&str, String> {
    let x = take_while1(|c: char| c.is_alphanumeric() || c == '_');
    map(x, |s: &str| s.to_string()).parse(input)
}
