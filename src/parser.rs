use crate::ast::Statement;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "sql.pest"]
pub struct SQLParser;

pub fn parse_sql(sql: &str) -> Result<Statement, Box<pest::error::Error<Rule>>> {
    let pairs = SQLParser::parse(Rule::statement, sql)?;
    let statement_pair = pairs.peek().unwrap();
    if statement_pair.as_rule() != Rule::statement {
        unreachable!();
    }
    let inner_statement = statement_pair.into_inner().peek().unwrap();
    Ok(match inner_statement.as_rule() {
        Rule::select_statement => {
            let mut inner_rules = inner_statement.into_inner();
            // The first rule is select_clause, then from_clause, then semicolon
            inner_rules.next(); // Consume the select_clause (SELECT *)
            let from_clause_pair = inner_rules.next().unwrap(); // This is the from_clause (FROM users)

            let mut from_inner_rules = from_clause_pair.into_inner();
            from_inner_rules.next(); // Consume the 'FROM' keyword
            let table_name = from_inner_rules.next().unwrap().as_str(); // This should be the identifier

            Statement::Select {
                table: table_name.to_string(),
            }
        }
        Rule::insert_statement => {
            let mut inner_rules = inner_statement.into_inner();
            inner_rules.next(); // INSERT
            inner_rules.next(); // INTO
            let table_name = inner_rules.next().unwrap().as_str(); // identifier
            let values_clause = inner_rules.next().unwrap(); // VALUES clause
            let mut values_inner_rules = values_clause.into_inner();
            for (i, p) in values_inner_rules.clone().enumerate() {
                eprintln!("[DEBUG] values_inner_rules[{}]: {:?}", i, p);
            }
            let _values_keyword = values_inner_rules.next(); // VALUES
            let value_list_pair = values_inner_rules.next().unwrap(); // value_list
            let values = value_list_pair
                .into_inner()
                .map(|p| p.as_str().trim_matches('\'').to_string())
                .collect();
            Statement::Insert {
                table: table_name.to_string(),
                values,
            }
        }
        Rule::update_statement => {
            let mut inner_rules = inner_statement.into_inner();
            inner_rules.next(); // UPDATE
            let table_name = inner_rules.next().unwrap().as_str(); // identifier
            let set_clause = inner_rules.next().unwrap(); // SET clause
            let mut set_inner_rules = set_clause.into_inner();
            set_inner_rules.next(); // Consume 'SET'
            let assignment_list_pair = set_inner_rules.next().unwrap(); // This is the assignment_list
            let assignments = assignment_list_pair
                .into_inner()
                .map(|p| {
                    let mut assignment_parts = p.into_inner();
                    let column = assignment_parts.next().unwrap().as_str();
                    let value = assignment_parts
                        .next()
                        .unwrap()
                        .as_str()
                        .trim_matches('\'')
                        .to_string();
                    (column.to_string(), value)
                })
                .collect();
            Statement::Update {
                table: table_name.to_string(),
                set: assignments,
            }
        }
        Rule::delete_statement => {
            let mut inner_rules = inner_statement.into_inner();
            inner_rules.next(); // DELETE
            inner_rules.next(); // FROM
            let table_name = inner_rules.next().unwrap().as_str(); // identifier
            Statement::Delete {
                table: table_name.to_string(),
            }
        }
        _ => unimplemented!(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Statement;

    #[test]
    fn test_parse_select_statement() {
        let sql = "SELECT * FROM users;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Select {
                table: "users".to_string()
            }
        );
    }

    #[test]
    fn test_parse_insert_statement() {
        let sql = "INSERT INTO users VALUES ('test_user', 'test_password');";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Insert {
                table: "users".to_string(),
                values: vec!["test_user".to_string(), "test_password".to_string()]
            }
        );
    }

    #[test]
    fn test_parse_update_statement() {
        let sql = "UPDATE users SET name = 'new_name', password = 'new_password';";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Update {
                table: "users".to_string(),
                set: vec![
                    ("name".to_string(), "new_name".to_string()),
                    ("password".to_string(), "new_password".to_string())
                ]
            }
        );
    }

    #[test]
    fn test_parse_delete_statement() {
        let sql = "DELETE FROM users;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Delete {
                table: "users".to_string(),
            }
        );
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
        let sql = "-- This is a comment
SELECT * FROM users;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_select_statement_lowercase() {
        let sql = "select * from users;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_insert_single_value() {
        let sql = "INSERT INTO users VALUES ('only_one');";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Insert {
                table: "users".to_string(),
                values: vec!["only_one".to_string()]
            }
        );
    }

    #[test]
    fn test_parse_update_single_assignment() {
        let sql = "UPDATE users SET name = 'foo';";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Update {
                table: "users".to_string(),
                set: vec![("name".to_string(), "foo".to_string())]
            }
        );
    }

    #[test]
    fn test_parse_table_name_with_underscore_and_number() {
        let sql = "SELECT * FROM user_01;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Select {
                table: "user_01".to_string()
            }
        );
    }

    #[test]
    fn test_parse_insert_value_with_underscore_and_number() {
        let sql = "INSERT INTO users VALUES ('foo_1', 'bar2');";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Insert {
                table: "users".to_string(),
                values: vec!["foo_1".to_string(), "bar2".to_string()]
            }
        );
    }

    #[test]
    fn test_parse_update_assignment_with_underscore_and_number() {
        let sql = "UPDATE users SET name_1 = 'foo', pass2 = 'bar';";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Update {
                table: "users".to_string(),
                set: vec![
                    ("name_1".to_string(), "foo".to_string()),
                    ("pass2".to_string(), "bar".to_string())
                ]
            }
        );
    }

    #[test]
    fn test_parse_insert_empty_value_list_should_fail() {
        let sql = "INSERT INTO users VALUES ();";
        let result = parse_sql(sql);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_update_empty_assignment_list_should_fail() {
        let sql = "UPDATE users SET ;";
        let result = parse_sql(sql);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_insert_value_with_single_quote_should_fail() {
        let sql = "INSERT INTO users VALUES ('foo\'bar');";
        let result = parse_sql(sql);
        assert!(result.is_err()); // エスケープ未対応のためエラーになるべき
    }

    #[test]
    fn test_parse_insert_value_with_japanese() {
        let sql = "INSERT INTO users VALUES ('ユーザー', 'パスワード');";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Statement::Insert {
                table: "users".to_string(),
                values: vec!["ユーザー".to_string(), "パスワード".to_string()]
            }
        );
    }
}
