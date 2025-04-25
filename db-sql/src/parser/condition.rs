use crate::parser::{Identifier, SqlValue, identifier, parse_sql_value};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag_no_case,
    character::char,
    character::complete::{space0, space1},
    combinator::{cut, map, opt},
    sequence::{delimited, preceded},
};

#[derive(Clone, PartialEq, Debug)]
pub enum Condition {
    Comparison {
        left: Identifier,
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
        map(tag_no_case("=="), |_| Operator::Equal),
        map(tag_no_case("="), |_| Operator::Equal),
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
fn parse_condition(input: &str) -> IResult<&str, Condition> {
    cut(alt((
        delimited(
            preceded(space0, char('(')),
            parse_condition,
            preceded(space0, char(')')),
        ),
        // map(
        //     (
        //         parse_condition,
        //         preceded(space1, parse_logical_operator),
        //         preceded(space1, parse_condition),
        //     ),
        //     |(f, operator, l)| Condition::Logical {
        //         operator,
        //         conditions: vec![f, l],
        //     },
        // ),
        map(
            (
                preceded(space1, identifier),
                preceded(space1, parse_operator),
                preceded(space1, parse_sql_value),
            ),
            |(left, operator, right)| Condition::Comparison {
                left,
                operator,
                right,
            },
        ),
    )))
    .parse(input)
}
pub fn parse_where_clause(input: &str) -> IResult<&str, Option<Condition>> {
    opt(preceded((space1, tag_no_case("WHERE")), parse_condition)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_where_clause() {
        let input = " WHERE id = 1";
        let (_, res) = parse_where_clause(input).unwrap();
        assert_eq!(
            res,
            Some(Condition::Comparison {
                left: "id".into(),
                operator: Operator::Equal,
                right: SqlValue::Integer(1)
            })
        )
    }
}
