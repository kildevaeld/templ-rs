pub(crate) mod chunk;
mod constant;
mod error;
mod opcode;
mod parse;
mod visitor;

pub use self::{constant::*, error::*, opcode::*};
use crate::template::Template;

pub fn compile<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Template>, CompileError> {
    let data = std::fs::read_to_string(path)?;
    let mut source = parse::parse(&data).unwrap();
    let runtime = templ_runtime::Runtime::new().build();

    let visitor = visitor::Visitor::new(runtime);

    visitor.compile(&mut source)
}
