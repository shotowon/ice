use crate::ast::{Expression, Statement, Type, TypeMapping};
use crate::tokens::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<String>> {
        let mut stmts: Vec<Statement> = Vec::new();
        let mut errs: Vec<String> = Vec::new();

        loop {
            match self.parse_stmt() {
                Ok(stmt) => {
                    if let Statement::Halt = stmt {
                        break;
                    }

                    stmts.push(stmt);
                }
                Err(err) => {
                    errs.push(err);
                    self.advance();
                }
            }
        }

        if errs.len() != 0 {
            return Err(errs);
        }

        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Statement, String> {
        if let Some(curr) = self.curr() {
            match curr.kind {
                TokenKind::EOF => return Ok(Statement::Halt),
                TokenKind::Return => {
                    self.advance();
                    if let Ok(_) = self.expect(TokenKind::Semicolon) {
                        self.advance();
                        return Ok(Statement::Return { value: None });
                    }

                    let expr = self.parse_expr()?;

                    self.expect(TokenKind::Semicolon)?;
                    self.advance();
                    return Ok(Statement::Return { value: Some(expr) });
                }
                _ => {
                    let expr = self.parse_expr()?;

                    match expr {
                        Expression::FunctionLiteral { .. } => {}
                        _ => {
                            self.expect(TokenKind::Semicolon)?;
                            self.advance();
                        }
                    }

                    return Ok(Statement::ExpressionStatement { expression: expr });
                }
            }
        }

        Err("expected statement before the end of input".into())
    }

    fn parse_expr(&mut self) -> Result<Expression, String> {
        self.parse_binary(0)
    }

    fn parse_binary(&mut self, min_bp: usize) -> Result<Expression, String> {
        let mut lhs = self.parse_prefix()?;

        while let Some(op) = self.curr() {
            if Self::is_binary_operator(op.kind) {
                let op = op.clone();
                let (l_bp, r_bp) = Self::get_binding_power(op.kind);

                if l_bp < min_bp {
                    break;
                }

                self.advance();

                let rhs = self.parse_binary(r_bp)?;

                lhs = Expression::Binary {
                    lhs: lhs.into(),
                    op: op.kind,
                    rhs: rhs.into(),
                }
            } else {
                break;
            }
        }

        return Ok(lhs);
    }

    fn parse_prefix(&mut self) -> Result<Expression, String> {
        if let Some(curr) = self.curr() {
            let curr = curr.clone();
            match curr.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    self.advance();
                    return Ok(Expression::Unary {
                        op: curr.kind,
                        expr: self.parse_atom()?.into(),
                    });
                }
                _ => {
                    return self.parse_atom();
                }
            }
        }

        Err("".into())
    }

    fn parse_atom(&mut self) -> Result<Expression, String> {
        if let Some(curr) = self.curr() {
            let curr = curr.clone();
            match curr.kind {
                TokenKind::Fn => {
                    return self.parse_function_literal_or_call();
                }
                TokenKind::Id => {
                    return self.parse_id_or_function_call();
                }
                TokenKind::Int => {
                    self.advance();
                    return Ok(Expression::Int { value: curr });
                }
                _ => {
                    return Err(format!(
                        "unexpected token '{}' ({:?}) at {}",
                        curr.literal, curr.kind, curr.location
                    ));
                }
            }
        }

        Err("unexpected end of input while parsing expression".into())
    }

    fn parse_id_or_function_call(&mut self) -> Result<Expression, String> {
        self.expect(TokenKind::Id)?;

        if matches!(self.peek(), Some(peek) if peek.kind.is(TokenKind::LParen)) {
            return self.parse_function_call();
        }

        if let Some(curr) = self.curr().cloned() {
            self.advance();
            return Ok(Expression::Id { name: curr.clone() });
        }

        Err("".into())
    }

    fn parse_function_call(&mut self) -> Result<Expression, String> {
        self.expect(TokenKind::Id)?;
        if let Some(curr) = self.curr() {
            let name = curr.clone();

            self.advance();
            self.expect(TokenKind::LParen)?;
            self.advance();

            let mut args: Vec<Expression> = Vec::new();

            while let Some(curr) = self.curr() {
                match curr.kind {
                    TokenKind::RParen => {
                        self.advance();
                        break;
                    }
                    _ => {
                        args.push(self.parse_expr()?);
                        if let Ok(_) = self.expect(TokenKind::Comma) {
                            self.advance();
                        }
                    }
                }
            }

            return Ok(Expression::FunctionCall {
                callee: Box::from(Expression::Id { name }),
                args,
            });
        }

        Err("expected identifier before function call".into())
    }

    fn parse_function_literal_or_call(&mut self) -> Result<Expression, String> {
        let fn_keyword = self.curr_expect(TokenKind::Fn)?.clone();
        self.advance();

        let mut name: Option<Token> = None;

        if let Ok(id) = self.curr_expect(TokenKind::Id) {
            name = Some(id.clone());
            self.advance();
        }

        self.expect(TokenKind::LParen)?;
        self.advance();

        let mut params: Vec<TypeMapping> = Vec::new();

        while let Err(_) = self.expect(TokenKind::RParen) {
            let param_name = self.curr_expect(TokenKind::Id)?.clone();
            self.advance();
            self.expect(TokenKind::Colon)?;
            self.advance();
            let param_type = self.parse_type()?;
            params.push(TypeMapping::new(
                Expression::Id {
                    name: param_name.clone(),
                },
                param_type,
            ));
            if let Ok(_) = self.expect(TokenKind::Comma) {
                self.advance();
            }
        }
        self.advance();

        let mut return_type: Option<Type> = None;
        if let Ok(_) = self.expect(TokenKind::Colon) {
            self.advance();
            self.expect(TokenKind::Colon)?;
            self.advance();

            return_type = Some(self.parse_type()?);
        }

        self.expect(TokenKind::LCurly)?;
        self.advance();

        let mut body: Vec<Statement> = Vec::new();

        while let Err(_) = self.expect(TokenKind::RCurly) {
            let stmt = self.parse_stmt()?;
            if let Statement::Halt = stmt {
                return Err(format!(
                    "unexpected end of input in function body at line: {}, col: {}",
                    fn_keyword.location.line, fn_keyword.location.col
                ));
            }

            body.push(stmt);
        }

        self.advance(); // skip }
        if let Ok(_) = self.expect(TokenKind::LParen) {
            self.advance();
            let mut args: Vec<Expression> = Vec::new();

            while let Some(curr) = self.curr() {
                match curr.kind {
                    TokenKind::RParen => {
                        self.advance();
                        break;
                    }
                    _ => {
                        args.push(self.parse_expr()?);
                        if let Ok(_) = self.expect(TokenKind::Comma) {
                            self.advance();
                        }
                    }
                }
            }

            return Ok(Expression::FunctionCall {
                callee: Expression::FunctionLiteral {
                    name,
                    params,
                    return_type,
                    body,
                }
                .into(),
                args: args,
            });
        }

        Ok(Expression::FunctionLiteral {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if let Some(curr) = self.curr() {
            match curr.kind {
                TokenKind::Int => {
                    self.advance();
                    return Ok(Type::Int);
                }
                _ => {
                    return Err(format!(
                        "Expected type at line: {}, col: {}, but got: {}",
                        curr.location.line, curr.location.col, curr.kind
                    ));
                }
            }
        }

        return Err("Expected type at the end of stream".into());
    }

    fn curr(&self) -> Option<&Token> {
        self.peek_off(0)
    }

    fn curr_expect(&self, kind: TokenKind) -> Result<&Token, String> {
        if let Some(curr) = self.curr() {
            if curr.kind != kind {
                return Err(format!(
                    "expected {:?} at line {} col {}",
                    kind, curr.location.line, curr.location.col
                ));
            }

            return Ok(curr);
        }

        if let Some(last) = self.tokens.last() {
            return Err(format!(
                "input expected {} after token at line {} col {} ",
                kind, last.location.line, last.location.col
            ));
        }

        Err(format!("input expected {}", kind))
    }

    fn expect(&self, kind: TokenKind) -> Result<(), String> {
        if let Some(curr) = self.curr() {
            if curr.kind != kind {
                return Err(format!(
                    "expected {:?} at line {} col {}",
                    kind, curr.location.line, curr.location.col
                ));
            }

            return Ok(());
        }

        if let Some(last) = self.tokens.last() {
            return Err(format!(
                "input expected {} after token at line {} col {} ",
                kind, last.location.line, last.location.col
            ));
        }

        Err(format!("input expected {}", kind))
    }

    fn expect_off(&self, kind: TokenKind, offset: usize) -> Result<(), String> {
        if let Some(token) = self.peek_off(offset) {
            if token.kind != kind {
                return Err(format!(
                    "expected {:?} at line {} col {}",
                    kind, token.location.line, token.location.col
                ));
            }

            return Ok(());
        }

        if let Some(last) = self.tokens.last() {
            return Err(format!(
                "input expected {} after token at line {} col {} ",
                kind, last.location.line, last.location.col
            ));
        }

        Err(format!("input expected {}", kind))
    }

    fn peek(&self) -> Option<&Token> {
        self.peek_off(1)
    }

    fn peek_off(&self, offset: usize) -> Option<&Token> {
        if self.pos + offset >= self.tokens.len() {
            return None;
        }

        (&self.tokens[self.pos + offset]).into()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn get_binding_power(op: TokenKind) -> (usize, usize) {
        match op {
            TokenKind::Plus | TokenKind::Minus => (1, 2),
            TokenKind::Star | TokenKind::Slash => (3, 4),
            _ => (0, 0),
        }
    }

    fn is_binary_operator(kind: TokenKind) -> bool {
        kind.is_one_of(&[
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
        ])
    }
}
