use crate::parser::{SqlDataType, SqlStatement, identifier};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag_no_case,
    character::complete::{space0, space1},
    combinator::map,
    sequence::preceded,
};

fn data_type(input: &str) -> IResult<&str, SqlDataType> {
    let (input, dtyp) = alt((
        tag_no_case("INT"),
        tag_no_case("STRING"),
        tag_no_case("TEXT"),
        tag_no_case("BIT"),
        tag_no_case("UUID"),
        tag_no_case("FLOAT"),
    ))
    .parse(input)?;

    let Ok(dtyp) = SqlDataType::try_from(dtyp) else {
        return Err(nom::Err::Failure(nom::error::make_error(
            input,
            nom::error::ErrorKind::Alpha,
        )));
    };
    Ok((input, dtyp))
}
fn constraint(input: &str) -> IResult<&str, String> {
    let (input, constraint) = alt((
        tag_no_case("DEFAULT"),
        tag_no_case("PRIMARY KEY"),
        tag_no_case("NOT NULL"),
        tag_no_case("UNIQUE"),
        tag_no_case("REFERENCES"),
        tag_no_case("CHECK"),
    ))
    .parse(input)?;
    Ok((input, constraint.to_string()))
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
        preceded(
            (
                space0,
                tag_no_case("CREATE"),
                space1,
                tag_no_case("TABLE"),
                space1,
            ),
            identifier,
        ),
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
