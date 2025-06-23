use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "sql.pest"]
pub struct SQLParser;

pub fn parse_sql(
    sql: &str,
) -> Result<pest::iterators::Pairs<'_, Rule>, Box<pest::error::Error<Rule>>> {
    SQLParser::parse(Rule::statement, sql).map_err(Box::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_select_statement() {
        let sql = "SELECT * FROM users;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_sql() {
        let sql = "SELECT FROM users;";
        let result = parse_sql(sql);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_whitespace() {
        let sql = "  SELECT   *   FROM   users  ;  ";
        let result = parse_sql(sql);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_comment() {
        let sql = "-- This is a comment\nSELECT * FROM users;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_select_statement_lowercase() {
        let sql = "select * from users;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
    }
}
