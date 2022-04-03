use crate::Token;

pub type Value = f64;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        value: Value,
    },
}
