use crate::parser::SqlValue;
use nom::branch::alt;
use nom::bytes::tag_no_case;
use nom::character::complete::space1;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

#[derive(Clone, PartialEq, Debug)]
pub enum Condition {
    Comparison {
        left: String,
        operator: Operator,
        right: SqlValue,
    },
    Logical {
        operator: LogicalOperator,
        conditions: Vec<Condition>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
#[derive(Clone, PartialEq, Debug, Eq, Copy)]
pub enum LogicalOperator {
    And,
    Or,
}
fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        map(tag_no_case("="), |_| Operator::Equal),
        map(tag_no_case("=="), |_| Operator::Equal),
        map(tag_no_case("!="), |_| Operator::NotEqual),
        map(tag_no_case("<="), |_| Operator::LessThanOrEqual),
        map(tag_no_case("<"), |_| Operator::LessThan),
        map(tag_no_case(">="), |_| Operator::GreaterThanOrEqual),
        map(tag_no_case(">"), |_| Operator::GreaterThan),
    ))
    .parse(input)
}
fn parse_logical_operator(input: &str) -> IResult<&str, LogicalOperator> {
    alt((
        map(tag_no_case("&&"), |_| LogicalOperator::And),
        map(tag_no_case("AND"), |_| LogicalOperator::And),
        map(tag_no_case("||"), |_| LogicalOperator::Or),
        map(tag_no_case("OR"), |_| LogicalOperator::Or),
    ))
    .parse(input)
}
fn parse_condition(input: &str) -> IResult<&str, Condition> {}
pub fn parse_where_clause(input: &str) -> IResult<&str, Option<Condition>> {
    opt(preceded((space1, tag_no_case("WHERE")), parse_condition)).parse(input)
}
