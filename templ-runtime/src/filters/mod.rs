use crate::{Args, Error, Value};
use std::fmt;
use std::sync::Arc;

pub trait Filter {
    fn name(&self) -> &str;
    fn args(&self) -> &Args;

    fn call(&self, args: &[&Value]) -> Result<Value, Error>;
}

#[derive(Clone)]
pub struct FilterBox(pub(crate) Arc<Box<dyn Filter>>);

impl FilterBox {
    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn call(&self, args: &[&Value]) -> Result<Value, Error> {
        self.0.call(args)
    }
}

pub struct FilterFn<F>(F, Args, String);

impl<F> FilterFn<F>
where
    F: 'static + Fn(&[&Value]) -> Result<Value, Error>,
{
    pub fn new(name: impl ToString, args: Args, cb: F) -> FilterFn<F> {
        FilterFn(cb, args, name.to_string())
    }
}

impl<F> Filter for FilterFn<F>
where
    F: 'static + Fn(&[&Value]) -> Result<Value, Error>,
{
    fn name(&self) -> &str {
        &self.2
    }
    fn args(&self) -> &Args {
        &self.1
    }

    fn call(&self, args: &[&Value]) -> Result<Value, Error> {
        (self.0)(args)
    }
}

impl PartialEq for FilterBox {
    fn eq(&self, other: &FilterBox) -> bool {
        self.0.name() == other.0.name()
    }
}

impl fmt::Debug for FilterBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Filter")
            .field("name", &self.0.name())
            .finish()
    }
}
