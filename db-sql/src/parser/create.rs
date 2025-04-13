use crate::parser::{identifier, ColumnDefinition, SqlDataType, SqlStatement};
use nom::{
    branch::alt, bytes::tag_no_case,
    character::char,
    character::complete::{space0, space1},
    combinator::map,
    combinator::opt,
    multi::separated_list1,
    sequence::delimited,
    sequence::preceded,
    IResult,
    Parser,
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

fn column_definition(input: &str) -> IResult<&str, ColumnDefinition> {
    map(
        (
            identifier,
            preceded(space1, data_type),
            opt(preceded(space1, constraint)),
        ),
        |(name, data_type, constraint)| ColumnDefinition {
            name: name.to_string(),
            data_type,
            constraint,
        },
    )
    .parse(input)
}
fn column_list(input: &str) -> IResult<&str, Vec<ColumnDefinition>> {
    separated_list1(
        delimited(space0, nom::character::complete::char(','), space0),
        column_definition,
    )
    .parse(input)
}
pub fn parse_create_statement(input: &str) -> IResult<&str, SqlStatement> {
    map(
        (
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
            preceded((space0, char('('), space0), column_list),
            space0,
            char(')'),
        ),
        |(table, columns, _, _)| SqlStatement::Create { table, columns },
    )
    .parse(input)
}
#[cfg(test)]
mod tests {
    use crate::parser::create::parse_create_statement;
    use crate::parser::{ColumnDefinition, SqlDataType, SqlStatement};

    #[test]
    fn should_parse_create_statement() {
        let input = "CREATE TABLE users ( id INT PRIMARY KEY, name STRING );";
        let (_, parsed) = parse_create_statement(input).unwrap();
        assert_eq!(
            parsed,
            SqlStatement::Create {
                table: "users".to_string(),
                columns: vec![
                    ColumnDefinition {
                        name: "id".into(),
                        data_type: SqlDataType::Integer,
                        constraint: Some("PRIMARY KEY".into()),
                    },
                    ColumnDefinition {
                        name: "name".into(),
                        data_type: SqlDataType::String,
                        constraint: None
                    },
                ],
            }
        );
    }
}
