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


fn identifier(input: &str) -> IResult<&str, String> {
    let x = take_while1(|c: char| c.is_alphanumeric() || c == '_');
    map(x, |s: &str| s.to_string()).parse(input)
}
