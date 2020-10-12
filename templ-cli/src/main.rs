use std::{env, fs};
use templ_parser;
use templ_runtime::{Number, Value};
use templ_vm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let o = env::args().skip(1).collect::<Vec<_>>();

    if o.is_empty() {
        eprintln!("usage templ-cli <path>");
        return Ok(());
    }

    let templates = templ_vm::compiler::compile_path(&o[0]).expect("compile");

    for t in templates {
        println!("{}", t);
        // let output = templ_vm::run_vm(
        //     &t,
        //     vec![
        //         Value::String("Hello".into()),
        //         Value::String("World".into()),
        //         Value::Number(Number::Integer(101.0)),
        //     ],
        // );

        // println!("{}", output);
    }

    // let data = fs::read_to_string(&o[0])?;
    // let ast = templ_parser::parse(&data)?;

    // let json = serde_json::to_string_pretty(&ast)?;

    // println!("{}", json);

    Ok(())
}
