#[derive(Debug, PartialEq)]
pub enum Statement {
    Select {
        table: String,
        where_clause: Option<Expression>,
    },
    Insert {
        table: String,
        values: Vec<String>,
    },
    Update {
        table: String,
        set: Vec<(String, String)>,
        where_clause: Option<Expression>,
    },
    Delete {
        table: String,
        where_clause: Option<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Column(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    // 比較演算子
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    // 論理演算子
    And,
    Or,
    // 算術演算子
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
}
