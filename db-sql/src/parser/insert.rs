use crate::parser::{SqlStatement, identifier};
use nom::{
    IResult, Parser,
    bytes::tag_no_case,
    character::complete::{space0, space1},
    combinator::{map, opt},
    sequence::preceded,
};

pub fn parse_insert_statement(input: &str) -> IResult<&str, SqlStatement> {
    map(
        (preceded(
            (
                space0,
                tag_no_case("INSERT"),
                space1,
                opt(tag_no_case("INTO")),
            ),
            identifier,
        )),
        |(table)| SqlStatement::Insert {
            table,
            values: vec![],
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_insert_statement() {
        let input = "INSERT INTO table1 VALUES (1,'email@gmail.com',true)";
        let (_, stat) = parse_insert_statement(input).unwrap();
        assert_eq!(
            stat,
            SqlStatement::Insert {
                table: "table1".into(),
                values: vec!["1".into(), "email@gmail.com".into(), "true".into()],
            }
        );
    }
}
