use crate::parser::{SqlStatement, SqlValue, identifier, parse_sql_value};
use nom::{
    IResult, Parser,
    bytes::tag_no_case,
    character::char,
    character::complete::{space0, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::delimited,
    sequence::preceded,
};

fn value_list(input: &str) -> IResult<&str, Vec<SqlValue>> {
    separated_list1(delimited(space0, char(','), space0), parse_sql_value).parse(input)
}
pub fn parse_insert_statement(input: &str) -> IResult<&str, SqlStatement> {
    map(
        (
            preceded(
                (
                    space0,
                    tag_no_case("INSERT"),
                    space1,
                    opt((tag_no_case("INTO"), space1)),
                ),
                identifier,
            ),
            preceded(
                (space1, tag_no_case("VALUES"), space0, char('('), space0),
                value_list,
            ),
            space0,
            char(')'),
        ),
        |(table, values, _, _)| SqlStatement::Insert { table, values },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::SqlValue;

    #[test]
    fn should_parse_insert_statement() {
        let input = "INSERT INTO users VALUES  (1, 'email@gmail.com', true, nil)";
        let (_, stat) = parse_insert_statement(input).unwrap();
        assert_eq!(
            stat,
            SqlStatement::Insert {
                table: "users".into(),
                values: vec![
                    SqlValue::Integer(1),
                    SqlValue::String("email@gmail.com".into()),
                    SqlValue::Boolean(true),
                    SqlValue::Nil,
                ]
            }
        );
    }
}
