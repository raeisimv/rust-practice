mod condition;
mod create;
mod delete;
mod insert;
mod select;

use condition::*;
use create::*;
use delete::*;
use insert::*;
use select::*;

use nom::{
    IResult, Parser, branch::alt, bytes::complete::take_while1, bytes::streaming::tag_no_case,
    character::char, combinator::map, sequence::delimited,
};

pub fn parse_sql(input: &str) -> IResult<&str, SqlStatement> {
    if let Ok(x) = parse_select_query(input.trim()) {
        Ok(x)
    } else if let Ok(x) = parse_create_statement(input) {
        Ok(x)
    } else if let Ok(x) = parse_delete_command(input) {
        Ok(x)
    } else if let Ok(x) = parse_insert_statement(input) {
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
        table: Identifier,
        columns: Vec<String>,
        condition: Option<Condition>,
    },
    Create {
        table: Identifier,
        columns: Vec<ColumnDefinition>,
    },
    Insert {
        table: Identifier,
        values: Vec<SqlValue>,
    },
    Delete {
        table: Identifier,
        condition: Option<Condition>,
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
    Nil,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnDefinition {
    pub name: Identifier,
    pub data_type: SqlDataType,
    pub constraint: Option<String>,
}

// #[derive(Clone, Debug, PartialEq)]
// pub enum Condition {
//     Equal(SqlValue),
//     NotEqual(SqlValue),
//     Greater(SqlValue),
//     Less(SqlValue),
//     GreaterEqual(SqlValue),
//     LessEqual(SqlValue),
// }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier(String);
impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}
fn identifier(input: &str) -> IResult<&str, Identifier> {
    let x = take_while1(|c: char| c.is_alphanumeric() || c == '_');
    map(x, Identifier::from).parse(input)
}

fn identifier_string(input: &str) -> IResult<&str, String> {
    let x = take_while1(|c: char| c.is_alphanumeric() || c == '_');
    map(x, |x: &str| x.into()).parse(input)
}
fn string_value(input: &str) -> IResult<&str, SqlValue> {
    map(
        delimited(
            char('\''),
            map(take_while1(|x| x != '\''), |s: &str| s.to_string()),
            char('\''),
        ),
        |x| SqlValue::String(x),
    )
    .parse(input)
}
fn boolean_value(input: &str) -> IResult<&str, SqlValue> {
    let (input, val) = alt((tag_no_case("true"), tag_no_case("false"))).parse(input)?;

    if val.to_lowercase() == "true" {
        Ok((input, SqlValue::Boolean(true)))
    } else {
        Ok((input, SqlValue::Boolean(false)))
    }
}
fn nil_value(input: &str) -> IResult<&str, SqlValue> {
    map(tag_no_case("NIL"), |_| SqlValue::Nil).parse(input)
}
fn int_value(input: &str) -> IResult<&str, SqlValue> {
    map(
        take_while1(|x: char| x.is_numeric() || x == '-' || x == '.'),
        |s: &str| {
            if s.contains('.') {
                SqlValue::Float(s.parse::<f64>().expect("Invalid Float"))
            } else {
                SqlValue::Integer(s.parse::<i32>().expect("Invalid Integer"))
            }
        },
    )
    .parse(input)
}
fn parse_sql_value(input: &str) -> IResult<&str, SqlValue> {
    alt((string_value, int_value, boolean_value, nil_value)).parse(input)
}
