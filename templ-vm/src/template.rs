use super::compiler::chunk::Chunk;
use std::fmt;
use templ_runtime::Type;

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
