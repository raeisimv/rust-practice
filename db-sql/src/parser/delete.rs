use crate::parser::SqlStatement;
use nom::IResult;

pub fn parse_delete_command(input: &str) -> IResult<&str, SqlStatement> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Condition, SqlValue};

    #[test]
    fn should_parse_delete_command() {
        let input = "DELETE FROM table WHERE id = 1";
        let (_, result) = parse_delete_command(input).unwrap();
        assert_eq!(
            result,
            SqlStatement::Delete {
                table: "table".into(),
                condition: vec![("id".into(), Condition::Equal(SqlValue::Integer(1)))]
            }
        )
    }
}
