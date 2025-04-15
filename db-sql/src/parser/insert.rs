use crate::parser::{identifier, SqlStatement};
use nom::bytes::take_while1;
use nom::character::char;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::{
    bytes::tag_no_case, character::complete::{space0, space1},
    combinator::{map, opt},
    sequence::preceded,
    IResult,
    Parser,
};

fn value_expression(input: &str) -> IResult<&str, String> {
    map(take_while1(|c: char| c != ','), String::from).parse(input)
}
fn value_list(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(delimited(space0, char(','), space0), value_expression).parse(input)
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

    #[test]
    fn should_parse_insert_statement() {
        let input = "INSERT INTO users VALUES (1,'email@gmail.com',true)";
        let (_, stat) = parse_insert_statement(input).unwrap();
        assert_eq!(
            stat,
            SqlStatement::Insert {
                table: "users".into(),
                values: vec!["1".into(), "email@gmail.com".into(), "true".into()],
            }
        );
    }
}
