use crate::tokens::{Token, TokenKind};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Statement {
    Return { value: Option<Expression> },
    ExpressionStatement { expression: Expression },
    Halt,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary {
        lhs: Box<Expression>,
        op: TokenKind,
        rhs: Box<Expression>,
    },
    Unary {
        op: TokenKind,
        expr: Box<Expression>,
    },
    FunctionCall {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    FunctionLiteral {
        name: Option<Token>,
        params: Vec<TypeMapping>,
        return_type: Option<Type>,
        body: Vec<Statement>,
    },
    Id {
        name: Token,
    },
    Int {
        value: Token,
    },
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    String,
    Function {
        return_type: Box<Type>,
        param_types: Vec<Type>,
    },
}

#[derive(Debug, Clone)]
pub struct TypeMapping {
    pub expr: Expression,
    pub t: Type,
}

impl TypeMapping {
    pub fn new(expr: Expression, t: Type) -> Self {
        Self { expr, t }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::ExpressionStatement { expression } => {
                write!(f, "{};", expression)
            }
            Statement::Return { value } => {
                if let Some(val) = value {
                    write!(f, "return {}", val)
                } else {
                    write!(f, "return")
                }
            }
            Statement::Halt => write!(f, "EOF"),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary { lhs, op, rhs } => {
                write!(f, "({} {} {})", lhs, op, rhs)
            }
            Expression::Unary { op, expr } => {
                write!(f, "({}{})", op, expr)
            }
            Expression::FunctionCall { callee, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "fcall: {}({})", callee, args_str.join(", "))
            }
            Expression::Id { name } => {
                write!(f, "{}", name.literal)
            }
            Expression::Int { value } => {
                write!(f, "{}", value.literal)
            }
            Expression::FunctionLiteral {
                name,
                params,
                return_type,
                body,
            } => {
                let name_str = name
                    .as_ref()
                    .map(|t| t.literal.clone())
                    .unwrap_or("<anon>".to_string());

                let params_str: Vec<String> = params
                    .iter()
                    .map(|p| format!("{}: {}", p.expr, p.t))
                    .collect();

                let ret_str = return_type
                    .as_ref()
                    .map(|t| format!("{}", t))
                    .unwrap_or("void".to_string());

                // Pretty-print body as a block
                let body_str: Vec<String> = body.iter().map(|stmt| format!("{}", stmt)).collect();

                write!(
                    f,
                    "fn {}({}) -> {} {{ {} }}",
                    name_str,
                    params_str.join(", "),
                    ret_str,
                    body_str.join(" ")
                )
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Function {
                return_type,
                param_types,
            } => {
                let params_str: Vec<String> = param_types.iter().map(|p| p.to_string()).collect();
                write!(f, "fn({}) -> {}", params_str.join(", "), return_type)
            }
        }
    }
}

impl fmt::Display for TypeMapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.expr, self.t)
    }
}
