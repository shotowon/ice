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
    Id {
        name: Token,
    },
    Int {
        value: Token,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::ExpressionStatement { expression } => {
                write!(f, "{};", expression)
            },
            Statement::Return { value } => {
                if let Some(val) = value {
                    write!(f, "return {}", val)
                } else {
                    write!(f, "return")
                }
            },
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
            Expression::FunctionCall { name, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "fcall: {}({})", name.literal, args_str.join(", "))
            }
            Expression::Id { name } => {
                write!(f, "{}", name.literal)
            }
            Expression::Int { value } => {
                write!(f, "{}", value.literal)
            }
        }
    }
}
