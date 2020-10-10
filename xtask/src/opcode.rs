use super::ast::read_file;
use std::error::Error;
use std::path::Path;
use tera::{Context, Tera};

pub fn generate() -> Result<String, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let def_path = path.join("definitions");
    let enums = read_file::<_, Vec<String>>(&def_path.join("op-codes.ron"))?;
    let p = format!("{}/*", path.join("templates").to_str().unwrap());
    let tera = Tera::new(&p)?;

    let mut ctx = Context::new();
    ctx.insert("opcodes", &enums);

    let out = tera.render("opcode.tera", &ctx)?;
    Ok(out)
}
