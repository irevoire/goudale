//! This module is dedicated to the definition and parsing of the [`Token`]s

use std::fmt::Display;

/// A [`Span`] represent a string from the source code with it's position
/// in the source as a byte offset. That's useful to generate good error
/// message.
#[derive(Debug, Clone, PartialEq)]
pub struct Span<'a> {
    source: &'a str,
    offset: usize,
    size: usize,
}

impl<'a> Span<'a> {
    pub fn new(source: &'a str, offset: usize, size: usize) -> Self {
        Self {
            source,
            offset,
            size,
        }
    }
}

impl<'a> From<&Span<'a>> for &'a str {
    fn from(span: &Span<'a>) -> Self {
        &span.source[span.offset..span.offset + span.size]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    span: Span<'a>,
    pub ty: TokenType,
}

impl<'a> Token<'a> {
    pub fn new(span: Span<'a>, ty: TokenType) -> Self {
        Self { span, ty }
    }

    pub fn lexeme(&self) -> &str {
        (&self.span).into()
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // single character token
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Slash,
    Star,

    // Literals
    Number,

    // Meta
    EoF,
}
