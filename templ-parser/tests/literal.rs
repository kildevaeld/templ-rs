use templ_ast::*;
use templ_parser;
#[test]
fn literal_string() {
    let ast = templ_parser::parse(
        r#"{% template Index() %}
{{ "Hello, World!" }}{% end %}"#,
    )
    .unwrap();

    assert_eq!(
        ModuleStmt {
            statements: vec![Stmt::Template(TemplateStmt::new(
                "Index".into(),
                Vec::default(),
                Box::new(Stmt::Block(BlockStmt::new(vec![Stmt::Expr(
                    ExprStmt::new(Expr::Literal(LiteralExpr::new(Literal::String(
                        "Hello, World!".into()
                    ))))
                )]))),
                None
            ))]
        },
        ast
    );
}

#[test]
fn literal_slice() {
    let ast = templ_parser::parse(
        r#"{% template Index() %}
{{  [200, "TEST"] }}{% end %}"#,
    )
    .unwrap();

    println!("AST {:?}", ast);
}

#[test]
fn literal_map() {
    let ast = templ_parser::parse(
        r#"{% template Index() %}
{{ { identifier: 200, "key": "Hello, World!" } }}{% end %}"#,
    )
    .unwrap();

    println!("AST {:?}", ast);
}
