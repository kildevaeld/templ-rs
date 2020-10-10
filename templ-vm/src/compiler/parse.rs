use templ_ast::ModuleStmt;

pub fn parse<'a>(source: &'a str) -> Result<ModuleStmt<'a>, Box<dyn std::error::Error>> {
    Ok(templ_parser::parse(source)?)
}
