mod error;
mod grammar;
mod visitor;

pub fn parse<'a>(content: &'a str) -> Result<templ_ast::ModuleStmt<'a>, error::Error> {
  let mut ast = grammar::templ_parser::parse(&content)?;
  let visitor = visitor::Visitor::new();
  visitor.visit(&mut ast)?;

  Ok(ast)
}

#[cfg(test)]
mod tests {

  use super::parse;
  #[test]
  fn test() {
    let data = include_str!("../simple.tpl");

    let out = parse(data).unwrap();

    println!("{:?}", out);
  }
}
