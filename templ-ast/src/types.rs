use super::ast::{Expr, Stmt};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Location(pub usize, pub usize);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Type<'a> {
    String,
    Bool,
    Number,
    Date,
    Slice(Box<Type<'a>>),
    User(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier<'a> {
    pub loc: Location,
    pub value: Cow<'a, str>,
}

impl<'a> Identifier<'a> {
    pub fn new(loc: Location, value: impl Into<Cow<'a, str>>) -> Identifier<'a> {
        Identifier {
            loc,
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Literal<'a> {
    String(Cow<'a, str>),
    Number(Number),
    Bool(bool),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Slice(Vec<Expr<'a>>),
    Map(HashMap<Cow<'a, str>, Expr<'a>>),
}

impl<'a> Literal<'a> {
    pub fn as_str(&self) -> Option<&Cow<'a, str>> {
        match self {
            Literal::String(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Number {
    Integer(f64),
    Double(f64),
}
