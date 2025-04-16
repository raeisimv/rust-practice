mod create;
mod insert;
mod select;

pub use create::*;
pub use select::*;

use nom::{
    character::char,
    bytes::streaming::tag_no_case,
    branch::alt,
    sequence::delimited,
    bytes::complete::take_while1,
    combinator::map,
    IResult,
    Parser
};

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

#[derive(Clone, Debug, PartialEq)]
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
        values: Vec<SqlValue>,
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

#[derive(Clone, Debug, PartialEq)]
pub enum SqlValue {
    String(String),
    Boolean(bool),
    Integer(i32),
    Float(f64),
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

fn string_value(input: &str) -> IResult<&str, String> {
    delimited(
        char('\''),
        map(take_while1(|x| x != '\''), |s: &str| s.to_string()),
        char('\''),
    )
    .parse(input)
}

fn boolean_value(input: &str) -> IResult<&str, bool> {
    let (input, val) = alt((tag_no_case("true"), tag_no_case("false"))).parse(input)?;

    if val.to_lowercase() == "true" {
        Ok((input, true))
    } else {
        Ok((input, false))
    }
}

fn int_value(input: &str) -> IResult<&str, i32> {
    map(take_while1(|x: char| x.is_numeric()), |s: &str| {
        s.parse::<i32>().unwrap_or_default()
    })
    .parse(input)
}
