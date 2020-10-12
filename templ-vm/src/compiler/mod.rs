pub(crate) mod chunk;
mod constant;
mod error;
mod opcode;
mod parse;
mod visitor;

pub use self::{constant::*, error::*, opcode::*};
use crate::template::Template;

use templ_runtime::Runtime;

pub fn compile_path<P: AsRef<std::path::Path>>(
    runtime: &Runtime,
    path: P,
) -> Result<Vec<Template>, CompileError> {
    let data = std::fs::read_to_string(&path)?;
    compile(runtime, data, Some(path.as_ref()))
}

pub fn compile<S: ToString>(
    runtime: &Runtime,
    source: S,
    path: Option<&std::path::Path>,
) -> Result<Vec<Template>, CompileError> {
    let visitor = visitor::Visitor::new(runtime.clone());
    let data = source.to_string();
    let mut source = parse::parse(&data).unwrap();
    visitor.compile(&mut source)
}
