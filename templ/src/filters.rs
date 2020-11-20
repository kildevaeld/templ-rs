use templ_runtime::{Args, FilterFn, RuntimeBuilder, Type, Value};

pub fn std_filters(runtime: RuntimeBuilder) -> RuntimeBuilder {
    runtime
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
}
