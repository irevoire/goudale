use crate::{Expr, ParserError, Scanner, Token, TokenType};

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    previous: Token<'a>,
    current: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Result<Self> {
        let mut scanner = Scanner::new(source);
        let current = scanner.scan_token()?;
        Ok(Self {
            scanner,
            previous: current.clone(),
            current,
        })
    }

    pub fn parse(mut self) -> Result<Expr<'a>> {
        let expr = self.expression()?;

        if self.is_at_end() {
            Ok(expr)
        } else {
            Err(ParserError::Tmp(format!(
                "Unexpected caracters `{:10?}` at the end of file.",
                self.scanner.rest(),
            )))
        }
    }

    fn expression(&mut self) -> Result<Expr<'a>> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr<'a>> {
        let mut expr = self.factor()?;

        while self.is_followed_by([TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous().clone();
            let right = Box::new(self.factor()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<'a>> {
        let mut expr = self.unary()?;

        while self.is_followed_by([TokenType::Star, TokenType::Slash])? {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<'a>> {
        if self.is_followed_by([TokenType::Minus])? {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);

            Ok(Expr::Unary { operator, right })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr<'a>> {
        let token = self.advance()?;
        match token.ty {
            TokenType::Number => Ok(Expr::Literal {
                value: token.lexeme().parse().unwrap(),
            }),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, ")")?;
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            ty => Err(ParserError::Tmp(format!(
                "Was expecting number but instead got {ty:?}",
            ))),
        }
    }

    fn advance(&mut self) -> Result<&Token<'a>> {
        if self.is_at_end() {
            Ok(self.peek())
        } else {
            self.previous = self.current.clone();
            self.current = self.scanner.scan_token()?;
            Ok(self.previous())
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::EoF
    }

    fn peek(&self) -> &Token<'a> {
        &self.current
    }

    fn previous(&self) -> &Token<'a> {
        &self.previous
    }

    fn check(&mut self, ty: &TokenType) -> bool {
        (!self.is_at_end()) && (&self.peek().ty == ty)
    }

    fn is_followed_by(&mut self, types: impl IntoIterator<Item = TokenType>) -> Result<bool> {
        for ty in types {
            if self.check(&ty) {
                self.advance()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn consume(&mut self, ty: &TokenType, expecting: impl AsRef<str>) -> Result<&Token<'a>> {
        if self.check(ty) {
            self.advance()
        } else {
            Err(ParserError::Consume(format!(
                "Got `{}`. Was expecting `{}`",
                self.peek().lexeme(),
                expecting.as_ref()
            )))
        }
    }
}
