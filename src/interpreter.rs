use crate::{Expr, InterpreterError, Value};

type Result<T> = std::result::Result<T, InterpreterError>;

impl Expr<'_> {
    pub fn interpret(&self) -> Result<Value> {
        match self {
            Expr::Unary { operator, right } => {
                let right = right.interpret()?;
                match operator.lexeme() {
                    "-" => Ok(-right),
                    _ => unreachable!(),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let (left, right) = (left.interpret()?, right.interpret()?);
                match operator.lexeme() {
                    "+" => Ok(left + right),
                    "-" => Ok(left - right),
                    "*" | "(" => Ok(left * right),
                    "/" => Ok(left / right),
                    _ => unreachable!(),
                }
            }
            Expr::Grouping { expression } => expression.interpret(),
            Expr::Literal { value } => Ok(value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Parser};

    #[test]
    fn test_value() -> Result<(), Error> {
        let res = Parser::new("1")?.parse()?.interpret()?;
        assert_eq!(res, 1.);
        let res = Parser::new("4000.53")?.parse()?.interpret()?;
        assert_eq!(res, 4000.53);
        Ok(())
    }

    #[test]
    fn test_unary() -> Result<(), Error> {
        let res = Parser::new("-1")?.parse()?.interpret()?;
        assert_eq!(res, -1.);
        Ok(())
    }

    #[test]
    fn test_binary() -> Result<(), Error> {
        let res = Parser::new("1 + 1")?.parse()?.interpret()?;
        assert_eq!(res, 2.);
        let res = Parser::new("5 - 1")?.parse()?.interpret()?;
        assert_eq!(res, 4.);
        let res = Parser::new("2 * 3")?.parse()?.interpret()?;
        assert_eq!(res, 6.);
        let res = Parser::new("6 / 3")?.parse()?.interpret()?;
        assert_eq!(res, 2.);
        let res = Parser::new("1 / 2")?.parse()?.interpret()?;
        assert_eq!(res, 0.5);
        let res = Parser::new("2 + 3 * 2")?.parse()?.interpret()?;
        assert_eq!(res, 8.);
        let res = Parser::new("2 + (3 * 2)")?.parse()?.interpret()?;
        assert_eq!(res, 8.);
        let res = Parser::new("2 * (3 + 2)")?.parse()?.interpret()?;
        assert_eq!(res, 10.);
        let res = Parser::new("2 (3 + 2)")?.parse()?.interpret()?;
        assert_eq!(res, 10.);
        Ok(())
    }
}
