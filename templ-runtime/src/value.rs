use super::{BlockBox, Error, RenderTarget, Renderable};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

pub trait ToType {
    fn to_type(&self) -> Type;
}

#[derive(Clone, PartialEq, Debug, Hash, Ord, Eq, PartialOrd)]
pub enum Type {
    Float,
    Integer,
    String,
    Bool,
    Date,
    Slice(Box<Type>),
    Struct(Vec<(String, Type)>),
    Block,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Number {
    Integer(f64),
    Float(f64),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    String(String),
    Number(Number),
    Bool(bool),
    Slice(Slice),
    Map(Map),
    Block(BlockBox),
}

impl Value {
    pub fn op_add(&self, value: &Value) -> Value {
        match (self, value) {
            (Value::Number(l), Value::Number(r)) => {
                //
                Value::Number(*l + *r)
            }

            _ => {
                let mut out = String::default();
                write!(out, "{}{}", self, value);
                Value::String(out)
            }
        }
    }

    pub fn op_gt(&self, value: &Value) -> bool {
        match (self, value) {
            (Value::Number(l), Value::Number(r)) => {
                //
                **l < **r
            }

            _ => {
                let mut out = String::default();
                write!(out, "{}{}", self, value);
                false
            }
        }
    }

    pub fn op_eq(&self, value: &Value) -> bool {
        self == value
    }

    pub fn as_boolean(&self) -> Option<bool> {
        let s = match self {
            Value::Bool(b) => *b,
            Value::String(s) => !s.is_empty(),
            Value::Number(_) => true,
            Value::Slice(s) => !s.data.is_empty(),
            Value::Map(m) => !m.inner.is_empty(),
            Value::Block(_) => true,
        };

        Some(s)
    }
}

impl ToType for Value {
    fn to_type(&self) -> Type {
        match self {
            Value::String(s) => Type::String,
            Value::Bool(b) => Type::Bool,
            Value::Number(n) => n.to_type(),
            Value::Slice(s) => s.to_type(),
            Value::Map(m) => m.to_type(),
            Value::Block(_) => Type::Block,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Map(_) => write!(f, "[Map]"),
            Value::Number(n) => write!(f, "{}", n),
            Value::Slice(s) => write!(f, "Slice"),
            Value::String(s) => write!(f, "{}", s),
            Value::Block(b) => write!(f, "Block({})", b.name()),
        }
    }
}

impl Renderable for Value {
    fn render(&self, dest: &mut dyn RenderTarget, args: &[Value]) -> Result<(), Error> {
        match self {
            Value::Block(b) => {
                b.render(dest, args)?;
            }
            _ => {
                write!(dest, "{}", self)?;
            }
        }
        Ok(())
    }
}

impl ToType for Number {
    fn to_type(&self) -> Type {
        match self {
            Number::Float(_) => Type::Float,
            Number::Integer(_) => Type::Integer,
        }
    }
}

impl std::ops::Deref for Number {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        match self {
            Number::Float(f) => f,
            Number::Integer(i) => i,
        }
    }
}

impl std::ops::Add for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Number {
        match self {
            Number::Integer(i) => Number::Integer(i + *rhs),
            Number::Float(f) => Number::Float(f + *rhs),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Float(d) => write!(f, "{}", d),
            Number::Integer(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Slice {
    ty: Type,
    data: Vec<Value>,
}

impl ToType for Slice {
    fn to_type(&self) -> Type {
        self.ty.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Map {
    inner: HashMap<String, Value>,
}

impl ToType for Map {
    fn to_type(&self) -> Type {
        let types = self
            .inner
            .iter()
            .map(|(k, v)| (k.clone(), v.to_type()))
            .collect();

        Type::Struct(types)
    }
}
