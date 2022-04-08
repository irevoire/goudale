use std::{
    fmt::Display,
    ops::{Add, Deref, Div, Mul, Neg, Sub},
};

use crate::Ty;

#[derive(Debug, Clone, PartialEq)]
pub struct Value<'a> {
    inner: f64,
    ty: Option<Ty<'a>>,
}

impl<'a> Value<'a> {
    pub fn new(inner: f64, ty: Option<Ty<'a>>) -> Self {
        Self { inner, ty }
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl PartialEq<f64> for Value<'_> {
    fn eq(&self, other: &f64) -> bool {
        self.inner == *other
    }
}

impl Deref for Value<'_> {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Add for Value<'a> {
    type Output = Value<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        // TODO: check the types
        Self::new(self.inner + rhs.inner, self.ty)
    }
}

impl<'a> Sub for Value<'a> {
    type Output = Value<'a>;

    fn sub(self, rhs: Self) -> Self::Output {
        // TODO: check the types
        Self::new(self.inner - rhs.inner, self.ty)
    }
}

impl<'a> Neg for Value<'a> {
    type Output = Value<'a>;

    fn neg(self) -> Self::Output {
        Self::new(-self.inner, self.ty)
    }
}

impl<'a> Mul for Value<'a> {
    type Output = Value<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        // TODO: check the types
        Self::new(self.inner * rhs.inner, self.ty)
    }
}

impl<'a> Div for Value<'a> {
    type Output = Value<'a>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.inner / rhs.inner, self.ty)
    }
}
