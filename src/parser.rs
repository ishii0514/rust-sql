use crate::ast::{
    BinaryOperator, Expression, GroupBy, Literal, OrderBy, OrderDirection, Statement, UnaryOperator,
};
use pest::{iterators::Pair, Parser};

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
            // The first rule is select_clause, then from_clause, then optional clauses, then semicolon
            inner_rules.next(); // Consume the select_clause (SELECT *)
            let from_clause_pair = inner_rules.next().unwrap(); // This is the from_clause (FROM users)

            let mut from_inner_rules = from_clause_pair.into_inner();
            from_inner_rules.next(); // Consume the 'FROM' keyword
            let table_name = from_inner_rules.next().unwrap().as_str(); // This should be the identifier

            // Parse optional clauses
            let mut where_clause = None;
            let mut group_by = None;
            let mut order_by = None;
            let mut limit = None;

            for clause in inner_rules {
                match clause.as_rule() {
                    Rule::where_clause => {
                        let mut where_inner = clause.into_inner();
                        where_inner.next(); // Consume WHERE keyword
                        let expr_pair = where_inner.next().unwrap();
                        where_clause = Some(build_expression_from_sql_parser(expr_pair));
                    }
                    Rule::group_by_clause => {
                        let mut group_by_inner = clause.into_inner();
                        group_by_inner.next(); // Consume GROUP keyword
                        group_by_inner.next(); // Consume BY keyword
                        let identifier_list = group_by_inner.next().unwrap(); // identifier_list
                        let columns = identifier_list
                            .into_inner()
                            .map(|p| p.as_str().to_string())
                            .collect();
                        group_by = Some(GroupBy { columns });
                    }
                    Rule::order_by_clause => {
                        let mut order_by_inner = clause.into_inner();
                        order_by_inner.next(); // Consume ORDER keyword
                        order_by_inner.next(); // Consume BY keyword
                        let column = order_by_inner.next().unwrap().as_str().to_string();
                        let direction = if let Some(dir_pair) = order_by_inner.next() {
                            match dir_pair.as_rule() {
                                Rule::order_direction => {
                                    let dir_inner = dir_pair.into_inner().next().unwrap();
                                    match dir_inner.as_rule() {
                                        Rule::ASC => OrderDirection::Asc,
                                        Rule::DESC => OrderDirection::Desc,
                                        _ => OrderDirection::Asc, // default
                                    }
                                }
                                _ => OrderDirection::Asc, // default
                            }
                        } else {
                            OrderDirection::Asc // default
                        };
                        order_by = Some(OrderBy { column, direction });
                    }
                    Rule::limit_clause => {
                        let mut limit_inner = clause.into_inner();
                        limit_inner.next(); // Consume LIMIT keyword
                        let limit_value = limit_inner.next().unwrap().as_str().parse().unwrap();
                        limit = Some(limit_value);
                    }
                    Rule::semicolon => {
                        // Skip semicolon
                    }
                    _ => {
                        // Skip other rules like semicolon
                    }
                }
            }

            Statement::Select {
                table: table_name.to_string(),
                where_clause,
                order_by,
                group_by,
                limit,
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

            // Check for optional WHERE clause
            let where_clause = if let Some(where_pair) = inner_rules.next() {
                if where_pair.as_rule() == Rule::where_clause {
                    let mut where_inner = where_pair.into_inner();
                    where_inner.next(); // Consume WHERE keyword
                    let expr_pair = where_inner.next().unwrap();
                    Some(build_expression_from_sql_parser(expr_pair))
                } else {
                    None
                }
            } else {
                None
            };

            Statement::Update {
                table: table_name.to_string(),
                set: assignments,
                where_clause,
            }
        }
        Rule::delete_statement => {
            let mut inner_rules = inner_statement.into_inner();
            inner_rules.next(); // DELETE
            inner_rules.next(); // FROM
            let table_name = inner_rules.next().unwrap().as_str(); // identifier

            // Check for optional WHERE clause
            let where_clause = if let Some(where_pair) = inner_rules.next() {
                if where_pair.as_rule() == Rule::where_clause {
                    let mut where_inner = where_pair.into_inner();
                    where_inner.next(); // Consume WHERE keyword
                    let expr_pair = where_inner.next().unwrap();
                    Some(build_expression_from_sql_parser(expr_pair))
                } else {
                    None
                }
            } else {
                None
            };

            Statement::Delete {
                table: table_name.to_string(),
                where_clause,
            }
        }
        _ => unimplemented!(),
    })
}

fn build_expression_from_sql_parser(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => {
            let inner = pair.into_inner().next().unwrap();
            build_expression_from_sql_parser(inner)
        }
        Rule::or_expression => {
            let mut inner = pair.into_inner();
            let mut expr = build_expression_from_sql_parser(inner.next().unwrap());

            while let Some(keyword) = inner.next() {
                if keyword.as_rule() == Rule::OR {
                    let right = build_expression_from_sql_parser(inner.next().unwrap());
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator: BinaryOperator::Or,
                        right: Box::new(right),
                    };
                }
            }
            expr
        }
        Rule::and_expression => {
            let mut inner = pair.into_inner();
            let mut expr = build_expression_from_sql_parser(inner.next().unwrap());

            while let Some(keyword) = inner.next() {
                if keyword.as_rule() == Rule::AND {
                    let right = build_expression_from_sql_parser(inner.next().unwrap());
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        operator: BinaryOperator::And,
                        right: Box::new(right),
                    };
                }
            }
            expr
        }
        Rule::equality_expression => {
            let mut inner = pair.into_inner();
            let mut expr = build_expression_from_sql_parser(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = match op_pair.as_rule() {
                    Rule::EQUAL => BinaryOperator::Equal,
                    Rule::NOT_EQUAL => BinaryOperator::NotEqual,
                    _ => unreachable!("Unexpected operator rule: {:?}", op_pair.as_rule()),
                };
                let right = build_expression_from_sql_parser(inner.next().unwrap());
                expr = Expression::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            }
            expr
        }
        Rule::comparison_expression => {
            let mut inner = pair.into_inner();
            let mut expr = build_expression_from_sql_parser(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = match op_pair.as_rule() {
                    Rule::LESS_THAN => BinaryOperator::LessThan,
                    Rule::LESS_THAN_OR_EQUAL => BinaryOperator::LessThanOrEqual,
                    Rule::GREATER_THAN => BinaryOperator::GreaterThan,
                    Rule::GREATER_THAN_OR_EQUAL => BinaryOperator::GreaterThanOrEqual,
                    _ => unreachable!("Unexpected operator rule: {:?}", op_pair.as_rule()),
                };
                let right = build_expression_from_sql_parser(inner.next().unwrap());
                expr = Expression::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            }
            expr
        }
        Rule::additive_expression => {
            let mut inner = pair.into_inner();
            let mut expr = build_expression_from_sql_parser(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = match op_pair.as_rule() {
                    Rule::ADD => BinaryOperator::Add,
                    Rule::SUBTRACT => BinaryOperator::Subtract,
                    _ => unreachable!("Unexpected operator rule: {:?}", op_pair.as_rule()),
                };
                let right = build_expression_from_sql_parser(inner.next().unwrap());
                expr = Expression::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            }
            expr
        }
        Rule::multiplicative_expression => {
            let mut inner = pair.into_inner();
            let mut expr = build_expression_from_sql_parser(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = match op_pair.as_rule() {
                    Rule::MULTIPLY => BinaryOperator::Multiply,
                    Rule::DIVIDE => BinaryOperator::Divide,
                    _ => unreachable!("Unexpected operator rule: {:?}", op_pair.as_rule()),
                };
                let right = build_expression_from_sql_parser(inner.next().unwrap());
                expr = Expression::Binary {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            }
            expr
        }
        Rule::unary_expression => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();

            match first.as_rule() {
                Rule::NOT => {
                    let operand = build_expression_from_sql_parser(inner.next().unwrap());
                    Expression::Unary {
                        operator: UnaryOperator::Not,
                        operand: Box::new(operand),
                    }
                }
                Rule::MINUS => {
                    let operand = build_expression_from_sql_parser(inner.next().unwrap());
                    Expression::Unary {
                        operator: UnaryOperator::Minus,
                        operand: Box::new(operand),
                    }
                }
                _ => build_expression_from_sql_parser(first),
            }
        }
        Rule::primary_expression => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::expression => build_expression_from_sql_parser(inner),
                _ => build_expression_from_sql_parser(inner),
            }
        }
        Rule::string_literal => {
            let content = pair.as_str();
            let trimmed = content.trim_matches('\'');
            Expression::Literal(Literal::String(trimmed.to_string()))
        }
        Rule::number_literal => {
            let num: i64 = pair.as_str().parse().unwrap();
            Expression::Literal(Literal::Number(num))
        }
        Rule::float_literal => {
            let num: f64 = pair.as_str().parse().unwrap();
            Expression::Literal(Literal::Float(num))
        }
        Rule::boolean_literal => {
            let is_true = pair.as_str().to_uppercase() == "TRUE";
            Expression::Literal(Literal::Boolean(is_true))
        }
        Rule::null_literal => Expression::Literal(Literal::Null),
        Rule::identifier => Expression::Column(pair.as_str().to_string()),
        _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
    }
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
                table: "users".to_string(),
                where_clause: None,
                order_by: None,
                group_by: None,
                limit: None,
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
                ],
                where_clause: None
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
                where_clause: None
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
                set: vec![("name".to_string(), "foo".to_string())],
                where_clause: None
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
                table: "user_01".to_string(),
                where_clause: None,
                order_by: None,
                group_by: None,
                limit: None,
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
                ],
                where_clause: None
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

    #[test]
    fn test_parse_select_with_where() {
        let sql = "SELECT * FROM users WHERE name = 'John';";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select {
                table,
                where_clause,
                order_by,
                group_by,
                limit,
            } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_some());
                assert!(order_by.is_none());
                assert!(group_by.is_none());
                assert!(limit.is_none());
                let expr = where_clause.unwrap();
                match expr {
                    Expression::Binary {
                        left,
                        operator,
                        right,
                    } => {
                        assert_eq!(*left, Expression::Column("name".to_string()));
                        assert_eq!(operator, BinaryOperator::Equal);
                        assert_eq!(
                            *right,
                            Expression::Literal(Literal::String("John".to_string()))
                        );
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_update_with_where() {
        let sql = "UPDATE users SET name = 'Jane' WHERE id = '1';";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Update {
                table,
                set,
                where_clause,
            } => {
                assert_eq!(table, "users");
                assert_eq!(set, vec![("name".to_string(), "Jane".to_string())]);
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_delete_with_where() {
        let sql = "DELETE FROM users WHERE active = 'false';";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Delete {
                table,
                where_clause,
            } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    #[test]
    fn test_parse_select_with_order_by_asc() {
        let sql = "SELECT * FROM users ORDER BY name ASC;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select {
                table,
                where_clause,
                order_by,
                group_by,
                limit,
            } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_none());
                assert!(group_by.is_none());
                assert!(limit.is_none());
                assert!(order_by.is_some());
                let order = order_by.unwrap();
                assert_eq!(order.column, "name");
                assert_eq!(order.direction, OrderDirection::Asc);
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_order_by_desc() {
        let sql = "SELECT * FROM users ORDER BY name DESC;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select {
                table,
                where_clause,
                order_by,
                group_by,
                limit,
            } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_none());
                assert!(group_by.is_none());
                assert!(limit.is_none());
                assert!(order_by.is_some());
                let order = order_by.unwrap();
                assert_eq!(order.column, "name");
                assert_eq!(order.direction, OrderDirection::Desc);
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_group_by() {
        let sql = "SELECT * FROM users GROUP BY department;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select {
                table,
                where_clause,
                order_by,
                group_by,
                limit,
            } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_none());
                assert!(order_by.is_none());
                assert!(limit.is_none());
                assert!(group_by.is_some());
                let group = group_by.unwrap();
                assert_eq!(group.columns, vec!["department"]);
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_group_by_multiple_columns() {
        let sql = "SELECT * FROM users GROUP BY department, status;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select {
                table,
                where_clause,
                order_by,
                group_by,
                limit,
            } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_none());
                assert!(order_by.is_none());
                assert!(limit.is_none());
                assert!(group_by.is_some());
                let group = group_by.unwrap();
                assert_eq!(group.columns, vec!["department", "status"]);
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_limit() {
        let sql = "SELECT * FROM users LIMIT 10;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select {
                table,
                where_clause,
                order_by,
                group_by,
                limit,
            } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_none());
                assert!(order_by.is_none());
                assert!(group_by.is_none());
                assert!(limit.is_some());
                assert_eq!(limit.unwrap(), 10);
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_all_extensions() {
        let sql = "SELECT * FROM users WHERE active = 'true' GROUP BY department ORDER BY name ASC LIMIT 10;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select {
                table,
                where_clause,
                order_by,
                group_by,
                limit,
            } => {
                assert_eq!(table, "users");
                assert!(where_clause.is_some());
                assert!(order_by.is_some());
                assert!(group_by.is_some());
                assert!(limit.is_some());

                let order = order_by.unwrap();
                assert_eq!(order.column, "name");
                assert_eq!(order.direction, OrderDirection::Asc);

                let group = group_by.unwrap();
                assert_eq!(group.columns, vec!["department"]);

                assert_eq!(limit.unwrap(), 10);
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_arithmetic_precedence() {
        let sql = "SELECT * FROM users WHERE 1 + 2 * 3 = 7;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select { where_clause, .. } => {
                let expr = where_clause.unwrap();
                assert_eq!(
                    expr,
                    Expression::Binary {
                        left: Box::new(Expression::Binary {
                            left: Box::new(Expression::Literal(Literal::Number(1))),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expression::Binary {
                                left: Box::new(Expression::Literal(Literal::Number(2))),
                                operator: BinaryOperator::Multiply,
                                right: Box::new(Expression::Literal(Literal::Number(3))),
                            }),
                        }),
                        operator: BinaryOperator::Equal,
                        right: Box::new(Expression::Literal(Literal::Number(7))),
                    }
                );
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_parentheses_in_arithmetic() {
        let sql = "SELECT * FROM users WHERE (1 + 2) * 3 = 9;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select { where_clause, .. } => {
                let expr = where_clause.unwrap();
                assert_eq!(
                    expr,
                    Expression::Binary {
                        left: Box::new(Expression::Binary {
                            left: Box::new(Expression::Binary {
                                left: Box::new(Expression::Literal(Literal::Number(1))),
                                operator: BinaryOperator::Add,
                                right: Box::new(Expression::Literal(Literal::Number(2))),
                            }),
                            operator: BinaryOperator::Multiply,
                            right: Box::new(Expression::Literal(Literal::Number(3))),
                        }),
                        operator: BinaryOperator::Equal,
                        right: Box::new(Expression::Literal(Literal::Number(9))),
                    }
                );
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_unary_minus_in_arithmetic() {
        let sql = "SELECT * FROM users WHERE -1 + 2 = 1;";
        let result = parse_sql(sql);
        assert!(result.is_ok());
        let statement = result.unwrap();
        match statement {
            Statement::Select { where_clause, .. } => {
                let expr = where_clause.unwrap();
                assert_eq!(
                    expr,
                    Expression::Binary {
                        left: Box::new(Expression::Binary {
                            left: Box::new(Expression::Unary {
                                operator: UnaryOperator::Minus,
                                operand: Box::new(Expression::Literal(Literal::Number(1))),
                            }),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expression::Literal(Literal::Number(2))),
                        }),
                        operator: BinaryOperator::Equal,
                        right: Box::new(Expression::Literal(Literal::Number(1))),
                    }
                );
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_invalid_arithmetic_missing_rhs() {
        let sql = "SELECT * FROM users WHERE 1 + ;";
        let result = parse_sql(sql);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_arithmetic_unclosed_paren() {
        let sql = "SELECT * FROM users WHERE (1 + 2;";
        let result = parse_sql(sql);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_arithmetic_double_operator() {
        let sql = "SELECT * FROM users WHERE 1 * * 2;";
        let result = parse_sql(sql);
        assert!(result.is_err());
    }
}
