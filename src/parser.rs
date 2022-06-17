use logos::{Lexer, Logos};

use crate::{Expr, ParserError, Token, TokenType, Ty};

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a, TokenType>,
    previous: Token<'a>,
    current: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = TokenType::lexer(source);
        let token = Token::new_from_lexer(&mut lexer);

        Self {
            lexer,
            previous: token.clone(),
            current: token,
        }
    }

    pub fn parse(mut self) -> Result<Expr<'a>> {
        let expr = self.expression()?;

        if self.is_at_end() {
            Ok(expr)
        } else {
            Err(ParserError::Tmp(format!(
                "Unexpected caracters `{:10?}` at the end of file.",
                self.lexer.remainder(),
            )))
        }
    }

    fn expression(&mut self) -> Result<Expr<'a>> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr<'a>> {
        let mut expr = self.factor()?;

        while self.is_followed_by([TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous.clone();
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

        while self.is_followed_by([TokenType::Star, TokenType::Slash, TokenType::LeftParen])? {
            let operator = self.previous.clone();
            let right;
            if operator.ty == TokenType::LeftParen {
                let expression = Box::new(self.expression()?);
                self.consume(&TokenType::RightParen, ")")?;
                right = Box::new(Expr::Grouping { expression });
            } else {
                right = Box::new(self.unary()?);
            }

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
            let operator = self.previous.clone();
            let right = Box::new(self.unary()?);

            Ok(Expr::Unary { operator, right })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr<'a>> {
        let token = self.advance()?;
        match token.ty {
            TokenType::Number => self.value(),
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

    fn value(&mut self) -> Result<Expr<'a>> {
        let value = self.previous.lexeme().parse().unwrap();

        let ty = if self.current.ty == TokenType::Type {
            Some(self.ty()?)
        } else {
            None
        };

        Ok(Expr::Literal {
            value: crate::Value::new(value, ty),
        })
    }

    fn ty(&mut self) -> Result<Ty<'a>> {
        // TODO: we need to somehow call a binary-op with only
        // the * and / symbol and type instead of value
        // for now we can stop the type parser once we encounter
        // a number.
        todo!()
    }

    fn advance(&mut self) -> Result<&Token<'a>> {
        if self.is_at_end() {
            Ok(&self.current)
        } else {
            self.previous = self.current.clone();
            self.current = Token::new_from_lexer(&mut self.lexer);
            Ok(&self.previous)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current.ty == TokenType::EoF
    }

    fn check(&mut self, ty: &TokenType) -> bool {
        &self.current.ty == ty
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
                self.current.lexeme(),
                expecting.as_ref()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ScannerError;

    use super::*;

    #[test]
    #[ignore]
    fn test_value() -> Result<()> {
        let expr = Parser::new("1").parse()?;
        assert!(matches!(expr, Expr::Literal { value } if value == 1.0));
        let expr = Parser::new("4000.53").parse()?;
        assert!(matches!(expr, Expr::Literal { value } if value == 4000.53 ));

        let result = Parser::new("4000.53.10").parse();
        assert!(matches!(
            result,
            Err(ParserError::Scanner(ScannerError::UnexpectedChar('.')))
        ));
        // Here we get the error before even calling parse because the
        // parser needs to call the scanner once to initialize it’s state
        let result = Parser::new("a").parse();
        assert!(matches!(result, Err(ParserError::Tmp(_))));
        let result = Parser::new("400a").parse();
        assert!(matches!(
            result,
            Err(ParserError::Scanner(ScannerError::UnexpectedChar('a')))
        ));

        Ok(())
    }

    #[test]
    fn test_unary() -> Result<()> {
        let expr = Parser::new("-1").parse()?;
        assert!(matches!(
            expr,
            Expr::Unary {
                operator,
                right: box Expr::Literal { value }
            } if operator.ty == TokenType::Minus && value == 1.
        ));
        Ok(())
    }
}
