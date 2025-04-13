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
        columns: Vec<ColumnDefinition>,
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
