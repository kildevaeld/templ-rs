use super::{compiler::chunk::Chunk, vm::run_vm, Error};
use std::fmt;
use templ_runtime::{RenderTarget, Type, Value};

pub struct Template {
    pub(crate) name: String,
    pub(crate) chunk: Chunk,
    pub(crate) params: Vec<Type>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Template: {}({:?})", self.name, self.params)?;
        write!(f, "{}", self.chunk)?;
        Ok(())
    }
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Template")
            .field("name", &self.name)
            .field("params", &self.params)
            .finish()
    }
}

impl Template {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params(&self) -> &Vec<Type> {
        &self.params
    }

    pub fn render_to_string(&self, args: Vec<Value>) -> Result<String, Error> {
        let mut output = String::default();
        self.render(&mut output, args)?;
        Ok(output)
    }

    pub fn render(&self, writer: &mut dyn RenderTarget, args: Vec<Value>) -> Result<(), Error> {
        run_vm(self, writer, args)
    }
}
