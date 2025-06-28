use crate::ast::{BinaryOperator, Expression, Literal, UnaryOperator};
use pest::{iterators::Pair, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "sql.pest"]
pub struct ExpressionParser;

pub fn parse_expression(expression_str: &str) -> Result<Expression, Box<pest::error::Error<Rule>>> {
    let pairs = ExpressionParser::parse(Rule::expression, expression_str)?;
    let expr_pair = pairs.into_iter().next().unwrap();
    Ok(build_expression(expr_pair))
}

pub fn build_expression(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => {
            let inner = pair.into_inner().next().unwrap();
            build_expression(inner)
        }
        Rule::or_expression => {
            let mut inner = pair.into_inner();
            let mut expr = build_expression(inner.next().unwrap());

            while let Some(keyword) = inner.next() {
                if keyword.as_rule() == Rule::OR {
                    let right = build_expression(inner.next().unwrap());
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
            let mut expr = build_expression(inner.next().unwrap());

            while let Some(keyword) = inner.next() {
                if keyword.as_rule() == Rule::AND {
                    let right = build_expression(inner.next().unwrap());
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
            let mut expr = build_expression(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = match op_pair.as_rule() {
                    Rule::EQUAL => BinaryOperator::Equal,
                    Rule::NOT_EQUAL => BinaryOperator::NotEqual,
                    _ => unreachable!("Unexpected operator rule: {:?}", op_pair.as_rule()),
                };
                let right = build_expression(inner.next().unwrap());
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
            let mut expr = build_expression(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = match op_pair.as_rule() {
                    Rule::LESS_THAN => BinaryOperator::LessThan,
                    Rule::LESS_THAN_OR_EQUAL => BinaryOperator::LessThanOrEqual,
                    Rule::GREATER_THAN => BinaryOperator::GreaterThan,
                    Rule::GREATER_THAN_OR_EQUAL => BinaryOperator::GreaterThanOrEqual,
                    _ => unreachable!("Unexpected operator rule: {:?}", op_pair.as_rule()),
                };
                let right = build_expression(inner.next().unwrap());
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
            let mut expr = build_expression(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = match op_pair.as_rule() {
                    Rule::ADD => BinaryOperator::Add,
                    Rule::SUBTRACT => BinaryOperator::Subtract,
                    _ => unreachable!("Unexpected operator rule: {:?}", op_pair.as_rule()),
                };
                let right = build_expression(inner.next().unwrap());
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
            let mut expr = build_expression(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = match op_pair.as_rule() {
                    Rule::MULTIPLY => BinaryOperator::Multiply,
                    Rule::DIVIDE => BinaryOperator::Divide,
                    _ => unreachable!("Unexpected operator rule: {:?}", op_pair.as_rule()),
                };
                let right = build_expression(inner.next().unwrap());
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
                    let operand = build_expression(inner.next().unwrap());
                    Expression::Unary {
                        operator: UnaryOperator::Not,
                        operand: Box::new(operand),
                    }
                }
                Rule::MINUS => {
                    let operand = build_expression(inner.next().unwrap());
                    Expression::Unary {
                        operator: UnaryOperator::Minus,
                        operand: Box::new(operand),
                    }
                }
                _ => build_expression(first),
            }
        }
        Rule::primary_expression => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::expression => build_expression(inner),
                _ => build_expression(inner),
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
    use crate::ast::{BinaryOperator, Expression, Literal, UnaryOperator};

    #[test]
    fn test_parse_string_literal() {
        let expr = "'hello'";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Literal(Literal::String("hello".to_string()))
        );
    }

    #[test]
    fn test_parse_number_literal() {
        let expr = "42";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expression::Literal(Literal::Number(42)));
    }

    #[test]
    fn test_parse_float_literal() {
        let expr = "3.14";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expression::Literal(Literal::Float(3.14)));
    }

    #[test]
    fn test_parse_boolean_literal_true() {
        let expr = "TRUE";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expression::Literal(Literal::Boolean(true)));
    }

    #[test]
    fn test_parse_boolean_literal_false() {
        let expr = "FALSE";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Literal(Literal::Boolean(false))
        );
    }

    #[test]
    fn test_parse_null_literal() {
        let expr = "NULL";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expression::Literal(Literal::Null));
    }

    #[test]
    fn test_parse_column_reference() {
        let expr = "name";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expression::Column("name".to_string()));
    }

    #[test]
    fn test_parse_equal_comparison() {
        let expr = "name = 'John'";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Column("name".to_string())),
                operator: BinaryOperator::Equal,
                right: Box::new(Expression::Literal(Literal::String("John".to_string())))
            }
        );
    }

    #[test]
    fn test_parse_not_equal_comparison() {
        let expr = "age != 25";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Column("age".to_string())),
                operator: BinaryOperator::NotEqual,
                right: Box::new(Expression::Literal(Literal::Number(25)))
            }
        );
    }

    #[test]
    fn test_parse_less_than_comparison() {
        let expr = "score < 100";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Column("score".to_string())),
                operator: BinaryOperator::LessThan,
                right: Box::new(Expression::Literal(Literal::Number(100)))
            }
        );
    }

    #[test]
    fn test_parse_and_expression() {
        let expr = "age > 18 AND score < 100";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Column("age".to_string())),
                    operator: BinaryOperator::GreaterThan,
                    right: Box::new(Expression::Literal(Literal::Number(18)))
                }),
                operator: BinaryOperator::And,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Column("score".to_string())),
                    operator: BinaryOperator::LessThan,
                    right: Box::new(Expression::Literal(Literal::Number(100)))
                })
            }
        );
    }

    #[test]
    fn test_parse_or_expression() {
        let expr = "status = 'active' OR status = 'pending'";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Column("status".to_string())),
                    operator: BinaryOperator::Equal,
                    right: Box::new(Expression::Literal(Literal::String("active".to_string())))
                }),
                operator: BinaryOperator::Or,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Column("status".to_string())),
                    operator: BinaryOperator::Equal,
                    right: Box::new(Expression::Literal(Literal::String("pending".to_string())))
                })
            }
        );
    }

    #[test]
    fn test_parse_not_expression() {
        let expr = "NOT active";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Unary {
                operator: UnaryOperator::Not,
                operand: Box::new(Expression::Column("active".to_string()))
            }
        );
    }

    #[test]
    fn test_parse_arithmetic_addition() {
        let expr = "price + tax";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Column("price".to_string())),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Column("tax".to_string()))
            }
        );
    }

    #[test]
    fn test_parse_arithmetic_subtraction() {
        let expr = "total - discount";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Column("total".to_string())),
                operator: BinaryOperator::Subtract,
                right: Box::new(Expression::Column("discount".to_string()))
            }
        );
    }

    #[test]
    fn test_parse_arithmetic_multiplication() {
        let expr = "quantity * price";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Column("quantity".to_string())),
                operator: BinaryOperator::Multiply,
                right: Box::new(Expression::Column("price".to_string()))
            }
        );
    }

    #[test]
    fn test_parse_arithmetic_division() {
        let expr = "total / count";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Binary {
                left: Box::new(Expression::Column("total".to_string())),
                operator: BinaryOperator::Divide,
                right: Box::new(Expression::Column("count".to_string()))
            }
        );
    }

    #[test]
    fn test_parse_unary_minus() {
        let expr = "-amount";
        let result = parse_expression(expr);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Expression::Unary {
                operator: UnaryOperator::Minus,
                operand: Box::new(Expression::Column("amount".to_string()))
            }
        );
    }

    #[test]
    fn test_parse_greater_than_or_equal() {
        let expr = "score >= 80";
        let result = parse_expression(expr);
        if result.is_err() {
            eprintln!("Parse error: {:?}", result.as_ref().err());
        }
        assert!(result.is_ok());
        let parsed = result.unwrap();
        eprintln!("Parsed result: {:?}", parsed);
        assert_eq!(
            parsed,
            Expression::Binary {
                left: Box::new(Expression::Column("score".to_string())),
                operator: BinaryOperator::GreaterThanOrEqual,
                right: Box::new(Expression::Literal(Literal::Number(80)))
            }
        );
    }

    #[test]
    fn test_parse_complex_expression() {
        let expr = "(age > 18 AND score >= 80) OR status = 'vip'";
        let result = parse_expression(expr);
        if result.is_err() {
            eprintln!("Parse error: {:?}", result.as_ref().err());
        }
        assert!(result.is_ok());
        // この複雑な式の構造も確認するテスト
    }
}
