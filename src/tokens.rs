use std::fmt;

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

#[derive(Clone, Debug)]
pub enum TokenKind {
    EOF,
    Plus,
    Minus,
    Star,
    Slash,
    Inc, // ++
    Decr, // decr
    Semicolon,
    LParen, //(
    RParen, // )
    Id,
    Int,
    Float,
    String,
    Println,
    Comma,
    Return,
}
