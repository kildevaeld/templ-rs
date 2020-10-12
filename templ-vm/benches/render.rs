use criterion::{black_box, criterion_group, criterion_main, Criterion};
use templ_runtime::{Number, Value};
use templ_vm;

static TEMPLATE: &'static str = include_str!("../../examples/simple.tpl");

static TERA_TEMPLATE: &'static str = include_str!("./tera.tpl");

fn template(_n: u64) {
    let templates = templ_vm::compiler::compile(TEMPLATE, None).unwrap();
    templ_vm::run_vm(
        &templates[0],
        vec![
            Value::String("Hello".into()),
            Value::String("World".into()),
            Value::Number(Number::Integer(101.0)),
        ],
    );
}

fn tera_template(_n: u64) {
    let mut ctx = tera::Context::default();
    ctx.insert("greeting", "Hello");
    ctx.insert("subject", "World");
    ctx.insert("age", &101);

    tera::Tera::one_off(TERA_TEMPLATE, &ctx, false).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Template", |b| b.iter(|| template(black_box(20))));
    c.bench_function("Tera Template", |b| b.iter(|| tera_template(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
