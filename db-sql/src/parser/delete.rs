use crate::parser::condition::parse_where_clause;
use crate::parser::{Condition, Identifier, SqlStatement, identifier};
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag_no_case;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;

fn condition(input: &str) -> IResult<&str, (Identifier, Condition)> {
    map(
        (space0, identifier, preceded(space1, condition)),
        |(_, _, x)| x,
    )
    .parse(input)
}
fn where_condition(input: &str) -> IResult<&str, Vec<(Identifier, Condition)>> {
    separated_list1(
        (space1, alt((tag_no_case("AND"), tag_no_case("OR"))), space1),
        condition,
    )
    .parse(input)
}
fn where_clause(input: &str) -> IResult<&str, Vec<(Identifier, Condition)>> {
    map(
        preceded((space1, tag_no_case("WHERE"), space1), where_condition),
        |x| x,
    )
    .parse(input)
}

pub fn parse_delete_command(input: &str) -> IResult<&str, SqlStatement> {
    map(
        (
            preceded((space0, tag_no_case("DELETE"), space1), identifier),
            parse_where_clause,
        ),
        |(table, condition)| SqlStatement::Delete { table, condition },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::condition::Operator;
    use crate::parser::{Condition, SqlValue};

    #[test]
    fn should_parse_delete_command() {
        let input = "DELETE users WHERE id == 1";
        let (_, result) = parse_delete_command(input).unwrap();
        assert_eq!(
            result,
            SqlStatement::Delete {
                table: "table".into(),
                condition: Some(Condition::Comparison {
                    left: "id".into(),
                    operator: Operator::Equal,
                    right: SqlValue::Integer(1)
                })
            }
        )
    }
}
