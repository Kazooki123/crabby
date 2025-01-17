use crate::lexer::{Token, TokenStream};
use crate::parser::ast::*;
use crate::utils::CrabbyError;

pub struct Parser<'a> {
    tokens: &'a [TokenStream<'a>],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [TokenStream]) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, CrabbyError> {
        let mut program = Program::new();
        while !self.is_at_end() {
            program.statements.push(self.parse_statement()?);
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, CrabbyError> {
        match &self.peek().token {
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

    fn parse_function_definition(&mut self) -> Result<Statement, CrabbyError> {
        self.advance(); // consume 'def'

        let name = if let Token::Identifier(name) = &self.peek().token {
            name.clone()
        } else {
            return Err(self.error("Expected function name"));
        };
        self.advance();

        self.consume(&Token::LParen, "Expected '(' after function name")?;
        
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
                self.consume(&Token::Comma, "Expected ',' between parameters")?;
            }
        }
        self.advance(); // consume ')'

        self.consume(&Token::Colon, "Expected ':' after parameters")?;
        let body = self.parse_block()?;

        Ok(Statement::FunctionDef {
            name,
            params,
            body: Box::new(body),
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, CrabbyError> {
        self.advance(); // consume 'if'
        let condition = self.parse_expression()?;
        self.consume(&Token::Colon, "Expected ':' after if condition")?;

        let then_branch = self.parse_block()?;

        let else_branch = if matches!(self.peek().token, Token::Else) {
            self.advance(); // consume 'else'
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

    fn parse_while_statement(&mut self) -> Result<Statement, CrabbyError> {
        self.advance(); // consume 'while'
        let condition = self.parse_expression()?;
        self.consume(&Token::Colon, "Expected ':' after while condition")?;
        let body = self.parse_block()?;

        Ok(Statement::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, CrabbyError> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Result<Expression, CrabbyError> {
        let mut expr = self.parse_multiplication()?;

        while matches!(self.peek().token, Token::Plus | Token::Minus) {
            let operator = match self.peek().token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_multiplication()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, CrabbyError> {
        let mut expr = self.parse_primary()?;

        while matches!(self.peek().token, Token::Star | Token::Slash) {
            let operator = match self.peek().token {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
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

    fn parse_primary(&mut self) -> Result<Expression, CrabbyError> {
        match &self.peek().token {
            Token::Integer(n) => {
                let n = *n;
                self.advance();
                Ok(Expression::Integer(n))
            }
            Token::Float(f) => {
                let f = *f;
                self.advance();
                Ok(Expression::Float(f))
            }
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expression::String(s))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                if matches!(self.peek().token, Token::LParen) {
                    self.parse_function_call(name)
                } else {
                    Ok(Expression::Variable(name))
                }
            }
            Token::Lambda => {
                self.advance(); // consume 'lambda'
                self.consume(&Token::LParen, "Expected '(' after lambda")?;
                
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
                        self.consume(&Token::Comma, "Expected ',' between parameters")?;
                    }
                }
                self.advance(); // consume ')'

                self.consume(&Token::Colon, "Expected ':' after parameters")?;
                let body = self.parse_block()?;

                Ok(Expression::Lambda {
                    params,
                    body: Box::new(body),
                })
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(&Token::RParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(self.error("Expected expression")),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, CrabbyError> {
        self.advance(); // consume 'let'

        let name = if let Token::Identifier(name) = &self.peek().token {
            name.clone()
        } else {
            return Err(self.error("Expected variable name"));
        };
        self.advance();

        self.consume(&Token::Equals, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;

        Ok(Statement::Let {
            name,
            value: Box::new(value),
        })
    }

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

        self.consume(&Token::RParen, "Expected ')' after arguments")?;

        Ok(Expression::Call {
            function: name,
            arguments,
        })
    }

    fn parse_block(&mut self) -> Result<Statement, CrabbyError> {
        self.consume(&Token::LBrace, "Expected '{' at start of block")?;

        let mut statements = Vec::new();
        while !matches!(self.peek().token, Token::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.consume(&Token::RBrace, "Expected '}' at end of block")?;
        Ok(Statement::Block(statements))
    }

    fn peek(&self) -> &TokenStream<'a> {
        if self.is_at_end() {
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

    fn consume(&mut self, expected: &Token, message: &str) -> Result<(), CrabbyError> {
        if self.peek().token == *expected {
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
}

pub fn parse(tokens: Vec<TokenStream>) -> Result<Program, CrabbyError> {
    let mut parser = Parser::new(&tokens);
    parser.parse()
}