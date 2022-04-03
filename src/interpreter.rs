use crate::{Expr, Value};

impl Expr<'_> {
    pub fn interpret(&self) -> Result<Value, ()> {
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
            Expr::Literal { value } => Ok(*value),
        }
    }
}
