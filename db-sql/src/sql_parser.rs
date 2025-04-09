use nom::{
    bytes::complete::take_while1, character::complete::char, combinator::map, IResult, Parser,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqlStatement {
    pub columns: Vec<String>,
    pub table: String,
    pub where_clause: Option<String>,
}

fn identifier(input: &str) -> IResult<&str, String> {
    let x = take_while1(|c: char| c.is_alphanumeric() || c == '_');
    map(x, |s: &str| s.to_string()).parse(input)
}
