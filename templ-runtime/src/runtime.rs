use super::{value::*, Error};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::sync::Arc;

pub struct Args(Vec<Type>);

pub trait RenderTarget: fmt::Write {}

impl RenderTarget for String {}

pub trait Filter {
    fn args(&self) -> &Args;

    fn call(&self, args: &[Value]) -> Result<Value, Error>;
}

#[derive(Clone)]
pub struct FilterBox(Arc<Box<dyn Filter>>);

pub trait Block: Renderable {
    fn name(&self) -> &str;
    fn args(&self) -> &Args;
}

#[derive(Clone)]
pub struct BlockBox(Arc<Box<dyn Block>>);

impl BlockBox {
    pub fn name(&self) -> &str {
        self.0.name()
    }
}

impl fmt::Debug for BlockBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Block")
            .field("name", &self.0.name())
            .finish()
    }
}

impl PartialEq for BlockBox {
    fn eq(&self, other: &BlockBox) -> bool {
        self.0.name() == other.0.name()
    }
}

impl Renderable for BlockBox {
    fn render(&self, dest: &mut dyn RenderTarget, args: &[Value]) -> Result<(), Error> {
        self.0.render(dest, args)
    }
}

pub trait Template: Renderable {
    fn args(&self) -> &Args;
}

#[derive(Clone)]
pub struct TemplateBox(Arc<Box<dyn Template>>);

pub trait Renderable {
    fn render(&self, dest: &mut dyn RenderTarget, args: &[Value]) -> Result<(), Error>;
}

pub struct RuntimeBuilder {
    filters: HashMap<String, FilterBox>,
    blocks: HashMap<String, BlockBox>,
    templates: HashMap<String, TemplateBox>,
}

impl RuntimeBuilder {
    pub fn build(self) -> Runtime {
        Runtime(Arc::new(RuntimeInner {
            filters: self.filters,
            blocks: self.blocks,
            templates: self.templates,
        }))
    }
}

struct RuntimeInner {
    filters: HashMap<String, FilterBox>,
    blocks: HashMap<String, BlockBox>,
    templates: HashMap<String, TemplateBox>,
}

#[derive(Clone)]
pub struct Runtime(Arc<RuntimeInner>);

impl Runtime {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            filters: HashMap::default(),
            blocks: HashMap::default(),
            templates: HashMap::default(),
        }
    }

    pub fn filter(&self, name: &str) -> Option<&FilterBox> {
        self.0.filters.get(name)
    }

    pub fn block(&self, name: &str) -> Option<&BlockBox> {
        self.0.blocks.get(name)
    }

    pub fn template(&self, name: &str) -> Option<&TemplateBox> {
        self.0.templates.get(name)
    }
}
