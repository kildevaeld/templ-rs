use getopts::Options;
use std::{env, fs};
use templ_parser;
use templ_runtime::{Args, FilterFn, Number, Runtime, Type, Value};
use templ_vm;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut opts = Options::new();

    opts.optflag("a", "ast", "Print ast")
        .optflagopt("b", "bytecode", "print", "")
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
    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(&args[0], opts);
        return Ok(());
    }

    let o = matches.free.clone();

    // let o = env::args().skip(1).collect::<Vec<_>>();

    // if o.is_empty() {
    //     eprintln!("usage templ-cli <path>");
    //     return Ok(());
    // }

    let runtime = Runtime::new()
        .filter(FilterFn::new(
            "uppercase",
            Args::new(vec![Type::String]),
            |args| {
                //
                let first = args.first().unwrap();
                Ok(Value::String(first.as_str()?.to_uppercase()))
            },
        ))
        .filter(FilterFn::new(
            "lowercase",
            Args::new(vec![Type::String]),
            |args| {
                //
                let first = args.first().unwrap();
                Ok(Value::String(first.as_str()?.to_lowercase()))
            },
        ))
        .build();

    let templates = templ_vm::compiler::compile_path(&runtime, &o[0]).expect("compile");

    for t in templates {
        // let mut out = String::new();
        // t.render(
        //     &mut out,
        //     vec![
        //         Value::String("Hello".into()),
        //         Value::String("World".into()),
        //         Value::Number(Number::Integer(101.0)),
        //     ],
        // );
        println!("{}", t);
    }

    Ok(())
}
