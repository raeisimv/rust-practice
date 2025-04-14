use crate::parser::SqlStatement;
use nom::IResult;

pub fn parse_insert_statement(input: &str) -> IResult<&str, SqlStatement> {
    todo!()
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
