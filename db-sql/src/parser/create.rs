use nom::{
    Parser,
    bytes::tag_no_case,
    character::complete::{space0, space1},
    combinator::map,
    sequence::preceded,
    IResult
};
use crate::parser::{identifier, SqlStatement};

#[derive(Copy, Clone, Debug)]
pub enum SqlDataType {
    Integer,
    String,
}
#[derive(Clone, Debug)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: SqlDataType,
}

// fn parse_column_definition(input: &str) -> IResult<&str, Vec<String>> {
//     separated_list1(delimited(
//         space0,
//         char(','),
//         identifier,
//     ))
//     .parse(input)
// }
pub fn parse_create_statement(input: &str) -> IResult<&str, SqlStatement> {
    map(
        (preceded((space0, tag_no_case("CREATE TABLE"), space1), identifier)),
        |(x)| SqlStatement::Create {
            table: x.to_string(),
        },
    )
    .parse(input)
}
#[cfg(test)]
mod tests {
    #[test]
    fn should_parse_create_statement() {
        let input = "CREATE TABLE users (id INTEGER, name STRING);";
    }
}
