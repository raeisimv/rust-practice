mod create;
mod insert;
mod select;

pub use create::*;
pub use select::*;

use nom::{IResult, Parser, bytes::complete::take_while1, combinator::map};

pub fn parse_sql(input: &str) -> IResult<&str, SqlStatement> {
    if let Ok(x) = parse_select_query(input.trim()) {
        Ok(x)
    } else if let Ok(x) = parse_create_statement(input) {
        Ok(x)
    } else {
        Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Fail,
        )))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SqlStatement {
    Select {
        table: String,
        columns: Vec<String>,
        condition: Option<String>,
    },
    Create {
        table: String,
        columns: Vec<ColumnDefinition>,
    },
    Insert {
        table: String,
        values: Vec<String>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SqlDataType {
    Integer,
    String,
    Boolean,
    Uuid,
    Text,
    Float,
}
impl TryFrom<&str> for SqlDataType {
    type Error = nom::Err<&'static str>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.to_uppercase().as_str() {
            "INT" => SqlDataType::Integer,
            "STRING" => SqlDataType::String,
            "UUID" => SqlDataType::Uuid,
            "TEXT" => SqlDataType::Text,
            "BIT" => SqlDataType::Boolean,
            "FLOAT" => SqlDataType::Float,
            _ => return Err(Self::Error::Failure("Invalid DataType")),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: SqlDataType,
    pub constraint: Option<String>,
}

fn identifier(input: &str) -> IResult<&str, String> {
    let x = take_while1(|c: char| c.is_alphanumeric() || c == '_');
    map(x, |s: &str| s.to_string()).parse(input)
}
