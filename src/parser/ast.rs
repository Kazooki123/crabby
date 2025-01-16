#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Statement>,
    },
    Let {
        name: String,
        value: Box<Expression>,
    },
    Return(Box<Expression>),
    If {
        condition: Box<Expression>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
    Block(Vec<Statement>),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    String(String),
    Variable(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
    },
    Call {
        function: String,
        arguments: Vec<Expression>,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}