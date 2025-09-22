use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub struct Location {
    pub col: usize,
    pub line: usize,
}

impl Location {
    pub fn new(col: usize, line: usize) -> Self {
        Self { col, line }
    }

    pub fn add_line(&mut self) {
        self.col = 1;
        self.line += 1;
    }

    pub fn add_col(&mut self) {
        self.col += 1;
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line: {}, col: {}", self.line, self.col)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub location: Location,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String, location: Location) -> Self {
        Self {
            kind,
            literal,
            location,
        }
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    EOF,
    Plus,
    Minus,
    Star,
    Slash,
    Inc, // ++
    Decr, // decr
    Colon,
    Semicolon,
    LParen, //(
    RParen, // )
    LCurly, // {
    RCurly, // }
    Id,
    Int,
    Float,
    String,
    Comma,
    Fn,
    Return,
}

impl TokenKind {
    pub fn is_one_of(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(self)
    }

    pub fn is(&self, kind: TokenKind) -> bool {
        kind == *self
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::EOF        => "end of file",
            TokenKind::Plus       => "+",
            TokenKind::Minus      => "-",
            TokenKind::Star       => "*",
            TokenKind::Slash      => "/",
            TokenKind::Inc        => "++",
            TokenKind::Decr       => "decr",
            TokenKind::Colon      => ":",
            TokenKind::Semicolon  => ";",
            TokenKind::LParen     => "(",
            TokenKind::RParen     => ")",
            TokenKind::LCurly     => "{",
            TokenKind::RCurly     => "}",
            TokenKind::Id         => "identifier",
            TokenKind::Int        => "integer literal",
            TokenKind::Float      => "float literal",
            TokenKind::String     => "string literal",
            TokenKind::Comma      => ",",
            TokenKind::Fn     =>  "fn",
            TokenKind::Return     => "return",
        };
        write!(f, "{}", s)
    }
}

