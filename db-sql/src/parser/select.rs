use crate::parser::{identifier, SqlStatement};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    bytes::tag_no_case,
    character::complete::char,
    character::complete::{space0, space1},
    combinator::map,
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, preceded},
};


fn column_list(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(delimited(space0, char(','), space0), identifier).parse(input)
}

fn where_clause(input: &str) -> IResult<&str, String> {
    preceded(
        (space0, tag_no_case("WHERE"), space1),
        map(take_while1(|x| x != ';'), |x: &str| x.to_string()),
    )
    .parse(input)
}

fn select_statement(input: &str) -> IResult<&str, SqlStatement> {
    map(
        (
            preceded((space0, tag_no_case("SELECT"), space1), column_list),
            preceded((space0, tag_no_case("FROM"), space1), identifier),
            opt(where_clause),
            space0,
            opt(char(';')),
        ),
        |(columns, table, condition, _, _)| SqlStatement::Select {
            table,
            columns,
            condition,
        },
    )
    .parse(input)
}

pub fn parse_select_query(input: &str) -> IResult<&str, SqlStatement> {
    alt((select_statement,)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_select_statement() {
        let input = "SELECT id, email, username FROM users WHERE id = 1;";
        let output = parse_select_query(input).unwrap();
        assert_eq!(
            output,
            (
                "",
                SqlStatement::Select {
                    table: "users".to_string(),
                    columns: vec!["id".into(), "email".into(), "username".into()],
                    condition: Some("id = 1".into()),
                }
            )
        );
    }
}
