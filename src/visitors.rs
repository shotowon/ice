use crate::ast::{Statement, Expression, TypeMapping, Type};
use crate::tokens::{TokenKind, Token};

pub trait SVisitor<R> {
    fn visit(&mut self, stmt: &Statement) -> Result<R, String> {
        match stmt {
            Statement::Return { value } => self.visit_return(value.as_ref()),
            Statement::ExpressionStatement { expression } => self.visit_expression_stmt(expression),
            Statement::Halt => self.visit_halt(),
        }
    }

    fn visit_return(&mut self, value: Option<&Expression>) -> Result<R, String>;
    fn visit_expression_stmt(&mut self, expr: &Expression) -> Result<R, String>;
    fn visit_halt(&mut self) -> Result<R, String>;
}

pub trait EVisitor<R> {
    fn visit(&mut self, expr: &Expression) -> Result<R, String> {
        match expr {
            Expression::Binary { lhs, op, rhs } => self.visit_binary(lhs, op, rhs),
            Expression::Unary { op, expr } => self.visit_unary(op, expr),
            Expression::FunctionCall { callee, args } => self.visit_function_call(callee, args),
            Expression::FunctionLiteral {
                name,
                params,
                return_type,
                body,
            } => self.visit_function_literal(name, &params, return_type, &body),
            Expression::Id { name } => self.visit_id(name.clone()),
            Expression::Int { value } => self.visit_int(value.clone()),
        }
    }

    fn visit_binary(&mut self, lhs: &Expression, op: &TokenKind, rhs: &Expression) -> Result<R, String>;
    fn visit_unary(&mut self, op: &TokenKind, expr: &Expression) -> Result<R, String>;
    fn visit_function_call(&mut self, callee: &Expression, args: &[Expression]) -> Result<R, String>;
    fn visit_function_literal(
        &mut self, 
        name: &Option<Token>,
        params: &[TypeMapping],
        return_type: &Option<Type>,
        body: &[Statement]
        ) -> Result<R, String>;
    fn visit_id(&mut self, name: Token) -> Result<R, String>;
    fn visit_int(&mut self, value: Token) -> Result<R, String>;
}
