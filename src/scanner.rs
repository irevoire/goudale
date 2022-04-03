use crate::{ScannerError, Span, Token, TokenType};

type Result<T> = std::result::Result<T, ScannerError>;

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    offset: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            offset: 0,
            current: 0,
        }
    }

    pub fn rest(self) -> &'a str {
        &self.source[self.offset..]
    }

    pub fn scan_token(&mut self) -> Result<Token<'a>> {
        self.skip_whitespace();
        self.offset = self.current;
        let current = self.advance();

        let token = match current {
            Some('(') => self.make_token(TokenType::LeftParen),
            Some(')') => self.make_token(TokenType::RightParen),
            Some('-') => self.make_token(TokenType::Minus),
            Some('+') => self.make_token(TokenType::Plus),
            Some('/') => self.make_token(TokenType::Slash),
            Some('*') => self.make_token(TokenType::Star),
            Some(c) if c.is_digit(10) => self.number(),
            Some(c) => unimplemented!("Unknown token {c:?}"),
            None => self.make_token(TokenType::EoF),
        };
        Ok(token)
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek().map(|c| c.is_digit(10)).unwrap_or(false) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().map(|c| c.is_digit(10)).unwrap_or(false) {
            self.advance();
            while self.peek().map(|c| c.is_digit(10)).unwrap_or(false) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\r' | '\t' | '\n') => drop(self.advance()),
                _ => return,
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().nth(0)
    }

    fn peek_next(&self) -> Option<char> {
        self.source[self.current..].chars().nth(1)
    }

    fn advance(&mut self) -> Option<char> {
        let current = self.peek()?;
        self.current += current.len_utf8();
        Some(current)
    }

    fn make_token(&self, ty: TokenType) -> Token<'a> {
        let span = Span::new(self.source, self.offset, self.current - self.offset);
        Token::new(span, ty)
    }
}
