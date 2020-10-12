use crate::{Args, Error, Value};
use std::sync::Arc;

pub trait Filter {
    fn args(&self) -> &Args;

    fn call(&self, args: &[Value]) -> Result<Value, Error>;
}

#[derive(Clone)]
pub struct FilterBox(pub(crate) Arc<Box<dyn Filter>>);

pub struct FilterFn<F>(F, Args);

impl<F> FilterFn<F> {
    pub fn new(args: Args, cb: F) -> FilterFn<F> {
        FilterFn(cb, args)
    }
}

impl<F> Filter for FilterFn<F>
where
    F: Fn(&[Value]) -> Result<Value, Error>,
{
    fn args(&self) -> &Args {
        &self.1
    }

    fn call(&self, args: &[Value]) -> Result<Value, Error> {
        (self.0)(args)
    }
}
