mod ast;
mod opcode;

use getopts::Options;
use std::env;
use std::path::Path;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn generate_ast() -> Result<(), Box<dyn std::error::Error>> {
    let out = ast::generate("ast.tera", None)?;

    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = path.parent().unwrap();
    let output = root.join("templ-ast/src/ast.rs");

    std::fs::write(output, &out)?;

    Ok(())
}

fn generate_visitor(name: &str, typed: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = tera::Context::default();
    ctx.insert("name", name);
    let out = ast::generate(
        if typed {
            "typed-visitor.tera"
        } else {
            "visitor.tera"
        },
        Some(ctx),
    )?;

    println!("{}", out);

    Ok(())
}

fn generate_opcode() -> Result<(), Box<dyn std::error::Error>> {
    let out = opcode::generate()?;

    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = path.parent().unwrap();
    let output = root.join("templ-vm/src/compiler/opcode.rs");

    std::fs::write(output, &out)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut opts = Options::new();

    // println!(
    //     "CUR {:?} {}",
    //     env::current_dir()?,
    //     env!("CARGO_MANIFEST_DIR")
    // );

    opts.optflag("a", "ast", "Generate ast structures")
        .optflagopt("v", "visitor", "description", "")
        .optflagopt("t", "typed visitor", "", "")
        .optflag("h", "help", "print usage");

    let args = env::args().collect::<Vec<_>>();

    if args.len() <= 1 {
        print_usage(&args[0], opts);
        return Ok(());
    }
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h")
    /*|| matches.free.is_empty()*/
    {
        print_usage(&args[0], opts);
        return Ok(());
    }

    if matches.opt_present("a") {
        generate_ast()?;
        // generate_typed_ast()?;
        generate_opcode()?;
    } else if let Some(name) = matches.opt_str("v") {
        generate_visitor(&name, false)?;
    } /*else if let Some(name) = matches.opt_str("t") {
          generate_visitor(&name, true)?;
      }*/

    Ok(())
}
