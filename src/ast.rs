use crate::tokens::{Token, TokenKind};
#[derive(Debug)]
pub enum Expression {
    Binary {
        lhs: Box<Expression>,
        op: TokenKind,
        rhs: Box<Expression>
    },
    Unary {
        op: TokenKind,
        expr: Box<Expression>,
    },
    FunctionCall {
        name: Token,
        args: Vec<Expression>,
    },
    Id {
        name: Token,
    },
    Int {
        value: Token,
    },
}
