use crate::{
    parser::condition::parse_where_clause,
    parser::{SqlStatement, identifier}
};
use nom::{
    character::complete::{space0, space1},
    bytes::tag_no_case,
    Parser,
    IResult,
    combinator::map,
    sequence::preceded
};

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
                table: "users".into(),
                condition: Some(Condition::Comparison {
                    left: "id".into(),
                    operator: Operator::Equal,
                    right: SqlValue::Integer(1)
                })
            }
        )
    }
}
