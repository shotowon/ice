use crate::tokens::{Location, Token, TokenKind};

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    location: Location,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src: src.chars().collect(),
            pos: 0,
            location: Location::new(1, 1),
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        while self.pos < self.src.len() {
            self.skip_whitespaces();
            self.skip_comments();
            if self.pos >= self.src.len() {
                break;
            }

            let curr = self.curr();

            if let Some(token) = self.lex_single_char_token() {
                self.tokens.push(token);
                self.advance();
                continue;
            }

            if curr == '"' {
                let token = self.lex_double_quoted_string()?;
                self.tokens.push(token);
                continue;
            }

            if curr.is_alphabetic() || curr == '_' {
                if let Some(token) = self.lex_id_or_keyword() {
                    self.tokens.push(token);
                    continue;
                }
            }

            if curr.is_numeric() {
                if let Some(token) = self.lex_number() {
                    self.tokens.push(token);
                    continue;
                }
            }

            return Err(format!("unrecognized lexeme at {}", self.location));
        }

        self.tokens
            .push(Token::new(TokenKind::EOF, "".into(), self.location.clone()));
        Ok(self.tokens.clone())
    }

    fn lex_single_char_token(&self) -> Option<Token> {
        let kind = match self.curr() {
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '+' => {
                if self.peek() == '+' {
                    TokenKind::Inc
                } else {
                    TokenKind::Plus
                }
            },
            '=' => {
                if self.peek() == '=' {
                    TokenKind::Eq2
                } else {
                    TokenKind::Eq
                }
            },
            '-' => {
                if self.peek() == '-' {
                    TokenKind::Decr
                } else {
                    TokenKind::Minus
                }
            }
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LCurly,
            '}' => TokenKind::RCurly,
            ',' => TokenKind::Comma,
            _ => {
                return None;
            }
        };

        Token::new(kind, self.curr().into(), self.location.clone()).into()
    }

    fn lex_double_quoted_string(&mut self) -> Result<Token, String> {
        if self.curr() != '"' {
            return Err("String must start from \"".into());
        }

        let location = self.location.clone();
        let mut literal = String::new();

        literal.push(self.curr());
        self.advance();

        while self.pos < self.src.len() {
            let curr = self.curr();
            if curr == '\n' {
                return Err("Unclosed string".into());
            }

            literal.push(curr);
            if curr == '"' && !Self::is_escaped(&literal) {
                self.advance();
                break;
            }

            self.advance();
        }

        Ok(Token::new(TokenKind::String, literal, location))
    }

    fn is_escaped(literal: &str) -> bool {
        let mut backslashes = 0;

        for ch in literal.chars().rev().skip(1) {
            if ch == '\\' {
                backslashes += 1;
            } else {
                break;
            }
        }

        backslashes % 2 == 1
    }

    fn lex_id_or_keyword(&mut self) -> Option<Token> {
        if !(self.curr().is_alphabetic() || self.curr() == '_') {
            return None;
        }

        let location = self.location.clone();
        let mut literal = String::new();

        while self.curr().is_alphanumeric() || self.curr() == '_' {
            literal.push(self.curr());
            self.advance();
        }

        let kind = Self::keyword_or_id_kind(&literal);

        Token::new(kind, literal, location).into()
    }

    fn keyword_or_id_kind(literal: &str) -> TokenKind {
        match literal {
            "return" => TokenKind::Return,
            "fn" => TokenKind::Fn,
            "int" => TokenKind::Int,
            _ => TokenKind::Id,
        }
    }

    fn lex_number(&mut self) -> Option<Token> {
        if !self.curr().is_numeric() {
            return None;
        }

        let location = self.location.clone();
        let mut literal = String::new();
        let mut is_float = false;

        while self.curr().is_numeric() || self.curr() == '_' || self.curr() == '.' {
            if self.curr() == '.' {
                is_float = true;
            }

            literal.push(self.curr());
            self.advance();
        }

        let mut kind = TokenKind::Int;

        if is_float {
            kind = TokenKind::Float;
        }

        Token::new(kind, literal, location).into()
    }

    fn curr(&self) -> char {
        if self.pos >= self.src.len() {
            return 0 as char;
        }

        self.src[self.pos]
    }
    fn peek(&self) -> char {
        if self.pos >= self.src.len() {
            return 0 as char;
        }

        self.src[self.pos + 1]
    }

    fn advance(&mut self) {
        if self.curr() == '\n' {
            self.location.add_line();
        } else {
            self.location.add_col();
        }
        self.pos += 1;
    }

    fn skip_comments(&mut self) {
        while self.curr() == '/' && (self.peek() == '*' || self.peek() == '/') {
            let curr = self.curr();
            let peek = self.peek();
            self.advance();
            self.advance();

            if curr == '/' && peek == '*' {
                while self.curr() != '*' && self.peek() != '/' && self.pos < self.src.len() {
                    self.advance();
                }
                self.advance();
                self.advance();
            } else {
                while self.curr() != '\n' && self.pos < self.src.len() {
                    self.advance();
                }
            }

            self.skip_whitespaces();
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.pos < self.src.len() {
            let curr = self.curr();
            if !curr.is_whitespace() {
                break;
            }

            self.advance();
        }
    }
}
