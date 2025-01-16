use crate::parser::ast::*;
use crate::utils::CrabbyError;
use std::collections::HashMap;

pub struct Compiler {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

#[derive(Clone)]
enum Value {
    Integer(i64),
    String(String),
    Lambda(Function),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Integer(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Lambda(_) => "<lambda>".to_string(),
        }
    }
}

#[derive(Clone)]
struct Function {
    params: Vec<String>,
    body: Box<Statement>,
}

impl Compiler {
    pub fn new() -> Self {
        let mut compiler = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };

        // Add built-in print function
        compiler.functions.insert("print".to_string(), Function {
            params: vec!["value".to_string()],
            body: Box::new(Statement::Expression(Expression::Variable("value".to_string()))),
        });
        
        compiler
    }

    fn handle_print(&mut self, args: &[Expression]) -> Result<Value, CrabbyError> {
        if args.len() != 1 {
            return Err(CrabbyError::CompileError("print takes exactly one argument".to_string()));
        }
        
        let value = self.compile_expression(&args[0])?;
        println!("{}", value.to_string());
        Ok(Value::Integer(0))
    }

    pub fn compile(&mut self, program: &Program) -> Result<(), CrabbyError> {
        for statement in &program.statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<Option<Value>, CrabbyError> {
        match statement {
            Statement::FunctionDef { name, params, body } => {
                self.functions.insert(name.clone(), Function {
                    params: params.clone(),
                    body: body.clone(),
                });
                Ok(None)
            }
            Statement::Let { name, value } => {
                let compiled_value = self.compile_expression(value)?;
                self.variables.insert(name.clone(), compiled_value);
                Ok(None)
            }
            Statement::Return(expr) => {
                let value = self.compile_expression(expr)?;
                Ok(Some(value))
            }
            Statement::If { condition, then_branch, else_branch } => {
                let condition_value = self.compile_expression(condition)?;
                match condition_value {
                    Value::Integer(0) => {
                        if let Some(else_branch) = else_branch {
                            self.compile_statement(else_branch)
                        } else {
                            Ok(None)
                        }
                    }
                    _ => self.compile_statement(then_branch),
                }
            }
            Statement::While { condition, body } => {
                loop {
                    let condition_value = self.compile_expression(condition)?;
                    match condition_value {
                        Value::Integer(0) => break,
                        _ => {
                            if let Some(Value::Integer(-1)) = self.compile_statement(body)? {
                                break;
                            }
                        }
                    }
                }
                Ok(None)
            }
            Statement::Block(statements) => {
                let mut last_value = None;
                for stmt in statements {
                    last_value = self.compile_statement(stmt)?;
                }
                Ok(last_value)
            }
            Statement::Expression(expr) => {
                let value = self.compile_expression(expr)?;
                Ok(Some(value))
            }
        }
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<Value, CrabbyError> {
        match expression {
            Expression::Call { function, arguments } => {
                if function == "print" {
                    return self.handle_print(arguments);
                }
            
                let func = self.functions.get(function).cloned().ok_or_else(|| {
                    CrabbyError::CompileError(format!("Undefined function: {}", function))
                })?;

                if arguments.len() != func.params.len() {
                    return Err(CrabbyError::CompileError(format!(
                        "Function {} expects {} arguments, got {}",
                        function,
                        func.params.len(),
                        arguments.len()
                    )));
                }

                let mut new_compiler = Compiler::new();
                for (param, arg) in func.params.iter().zip(arguments) {
                    let arg_value = self.compile_expression(arg)?;
                    new_compiler.variables.insert(param.clone(), arg_value);
                }

                match new_compiler.compile_statement(&func.body)? {
                    Some(value) => Ok(value),
                    None => Ok(Value::Integer(0)),
                }
            }
            Expression::Integer(n) => Ok(Value::Integer(*n)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Variable(name) => {
                self.variables.get(name).cloned().ok_or_else(|| {
                    CrabbyError::CompileError(format!("Undefined variable: {}", name))
                })
            }
            Expression::Binary { left, operator, right } => {
                let left_val = self.compile_expression(left)?;
                let right_val = self.compile_expression(right)?;
            
                match (left_val, operator, right_val) {
                    (Value::Integer(l), BinaryOp::Add, Value::Integer(r)) => Ok(Value::Integer(l + r)),
                    (Value::String(l), BinaryOp::Add, Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                    (Value::String(l), BinaryOp::Add, r) => Ok(Value::String(format!("{}{}", l, r.to_string()))),
                    (l, BinaryOp::Add, Value::String(r)) => Ok(Value::String(format!("{}{}", l.to_string(), r))),
                    (Value::Integer(l), BinaryOp::Sub, Value::Integer(r)) => Ok(Value::Integer(l - r)),
                    (Value::Integer(l), BinaryOp::Mul, Value::Integer(r)) => Ok(Value::Integer(l * r)),
                    (Value::Integer(l), BinaryOp::Div, Value::Integer(r)) => {
                        if r == 0 {
                            return Err(CrabbyError::CompileError("Division by zero".to_string()));
                        }
                        Ok(Value::Integer(l / r))
                    }
                    (Value::Integer(l), BinaryOp::Eq, Value::Integer(r)) => {
                        Ok(Value::Integer(if l == r { 1 } else { 0 }))
                    }
                    _ => Err(CrabbyError::CompileError("Invalid operation".to_string())),
                }
            }
            Expression::Lambda { params, body } => {
                Ok(Value::Lambda(Function {
                    params: params.clone(),
                    body: body.clone(),
                }))
            }
        }
    }
}

pub fn compile(ast: &Program) -> Result<(), CrabbyError> {
    let mut compiler = Compiler::new();
    compiler.compile(ast)
}
