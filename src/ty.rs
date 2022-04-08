use crate::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty<'a> {
    Modifier(TypeModifier<'a>),
}

#[derive(Debug, Clone)]
pub enum TypeModifier<'a> {
    Kilo(Token<'a>),
}

impl PartialEq for TypeModifier<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for TypeModifier<'_> {}
