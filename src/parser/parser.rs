use crate::lexer::{Token, TokenStream};
use crate::parser::ast::*;
use crate::utils::CrabbyError;

pub struct Parser<'a> {
    tokens: &'a [TokenStream<'a>],
    current: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser instance
    pub fn new(tokens: &'a [TokenStream]) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    /// Parses the complete program
    pub fn parse(&mut self) -> Result<Program, CrabbyError> {
        let mut program = Program::new();
        while !self.is_at_end() {
            program.statements.push(self.parse_statement()?);
        }
        Ok(program)
    }

    /// Parses a single statement
    fn parse_statement(&mut self) -> Result<Statement, CrabbyError> {
        match self.peek().token {
            Token::Def => self.parse_function_definition(),
            Token::Let => self.parse_let_statement(),
            Token::Return => {
                self.advance(); // consume 'return'
                let expr = self.parse_expression()?;
                Ok(Statement::Return(Box::new(expr)))
            },
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            _ => {
                let expr = self.parse_expression()?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    /// Parses a function definition: def name(params): { body }
    fn parse_function_definition(&mut self) -> Result<Statement, CrabbyError> {
        self.advance(); // consume 'def'

        // Parse function name
        let name = if let Token::Identifier(name) = &self.peek().token {
            name.clone()
        } else {
            return Err(self.error("Expected function name"));
        };
        self.advance();

        // Parse parameters
        self.consume(Token::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();

        if !matches!(self.peek().token, Token::RParen) {
            loop {
                if let Token::Identifier(param) = &self.peek().token {
                    params.push(param.clone());
                    self.advance();
                } else {
                    return Err(self.error("Expected parameter name"));
                }

                if matches!(self.peek().token, Token::RParen) {
                    break;
                }
                self.consume(Token::Comma, "Expected ',' between parameters")?;
            }
        }
        self.advance(); // consume ')'

        // Parse function body
        self.consume(Token::Colon, "Expected ':' after parameters")?;
        let body = self.parse_block()?;

        Ok(Statement::FunctionDef {
            name,
            params,
            body: Box::new(body),
        })
    }

    /// Parses a let statement: let name = expression
    fn parse_let_statement(&mut self) -> Result<Statement, CrabbyError> {
        self.advance(); // consume 'let'

        let name = if let Token::Identifier(name) = &self.peek().token {
            name.clone()
        } else {
            return Err(self.error("Expected variable name"));
        };
        self.advance();

        self.consume(Token::Equals, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;

        Ok(Statement::Let {
            name,
            value: Box::new(value),
        })
    }

    /// Parses an if statement: if condition: { body } else: { body }
    fn parse_if_statement(&mut self) -> Result<Statement, CrabbyError> {
        self.advance(); // consume 'if'
        let condition = self.parse_expression()?;
        self.consume(Token::Colon, "Expected ':' after if condition")?;

        let then_branch = self.parse_block()?;

        let else_branch = if matches!(self.peek().token, Token::Else) {
            self.advance(); // consume 'else'
            self.consume(Token::Colon, "Expected ':' after else")?;
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    /// Parses a while statement: while condition: { body }
    fn parse_while_statement(&mut self) -> Result<Statement, CrabbyError> {
        self.advance(); // consume 'while'
        let condition = self.parse_expression()?;
        self.consume(Token::Colon, "Expected ':' after while condition")?;
        let body = self.parse_block()?;

        Ok(Statement::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    /// Parses a block of statements: { statement* }
    fn parse_block(&mut self) -> Result<Statement, CrabbyError> {
        self.consume(Token::LBrace, "Expected '{' at start of block")?;

        let mut statements = Vec::new();
        while !matches!(self.peek().token, Token::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(Token::RBrace, "Expected '}' at end of block")?;
        Ok(Statement::Block(statements))
    }

    /// Parses an expression starting from the lowest precedence operators
    fn parse_expression(&mut self) -> Result<Expression, CrabbyError> {
        self.parse_binary_expression()
    }

    /// Parses binary expressions with operator precedence
    fn parse_binary_expression(&mut self) -> Result<Expression, CrabbyError> {
        let mut expr = self.parse_primary()?;

        while matches!(self.peek().token,
            Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::DoubleEquals |
            Token::Dot
        ) {
            let operator = match self.peek().token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::DoubleEquals => BinaryOp::Eq,
                Token::Dot => BinaryOp::Dot,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_primary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses primary expressions (literals, variables, function calls, etc.)
    fn parse_primary(&mut self) -> Result<Expression, CrabbyError> {
        let token = self.peek().token.clone();
        match token {
            Token::Integer(n) => {
                self.advance();
                Ok(Expression::Integer(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expression::String(s))
            }
            Token::Identifier(name) => {
                self.advance();
                if matches!(self.peek().token, Token::LParen) {
                    self.parse_function_call(name)
                } else {
                    Ok(Expression::Variable(name))
                }
            }
            Token::Lambda => self.parse_lambda(),
            _ => Err(self.error("Expected expression")),
        }
    }
    /// Parses a function call: name(args)
    fn parse_function_call(&mut self, name: String) -> Result<Expression, CrabbyError> {
        self.advance(); // consume '('

        let mut arguments = Vec::new();
        if !matches!(self.peek().token, Token::RParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if !matches!(self.peek().token, Token::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
        }

        self.consume(Token::RParen, "Expected ')' after arguments")?;

        Ok(Expression::Call {
            function: name,
            arguments,
        })
    }

    /// Parses a lambda expression: lambda(params): { body }
    fn parse_lambda(&mut self) -> Result<Expression, CrabbyError> {
        self.advance(); // consume 'lambda'

        self.consume(Token::LParen, "Expected '(' after lambda")?;

        let mut params = Vec::new();
        if !matches!(self.peek().token, Token::RParen) {
            loop {
                if let Token::Identifier(name) = &self.peek().token {
                    params.push(name.clone());
                    self.advance();
                }

                if !matches!(self.peek().token, Token::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
        }

        self.consume(Token::RParen, "Expected ')' after lambda parameters")?;
        self.consume(Token::Colon, "Expected ':' after lambda parameters")?;

        let body = self.parse_block()?;

        Ok(Expression::Lambda {
            params,
            body: Box::new(body),
        })
    }

    fn peek(&self) -> &TokenStream<'a> {
        if self.is_at_end() {
            // Return last token if we're at the end
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.current]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }


    fn consume(&mut self, expected: Token, message: &str) -> Result<(), CrabbyError> {
        if self.is_at_end() {
            return Err(self.error(&format!("Unexpected end of file. {}", message)));
        }

        if self.peek().token == expected {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&self, message: &str) -> CrabbyError {
        let span = if self.is_at_end() {
            &self.tokens[self.tokens.len() - 1].span
        } else {
            &self.peek().span
        };

        CrabbyError::ParserError {
            line: span.line,
            column: span.column,
            message: message.to_string(),
        }
    }

    // Add this helper method for better token matching
    fn check(&self, token_type: Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token == token_type
        }
    }
}

/// Entry point for parsing a program
pub fn parse(tokens: Vec<TokenStream>) -> Result<Program, CrabbyError> {
    let mut parser = Parser::new(&tokens);
    parser.parse()
}
