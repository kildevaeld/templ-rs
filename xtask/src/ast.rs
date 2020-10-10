use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
enum Type {
    String,
    Bool,
    Expr,
    Stmt,
    Type,
    Identifier,
    Literal,
    AssignmentOperator,
    BinaryOperator,
    LogicalOperator,
    UnaryOperator,
    PostfixOperator,
    Argument,
    // ClassMember,
    Optional(Box<Type>),
    Repeated(Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "Cow<'a, str>"),
            Type::Expr => write!(f, "Expr<'a>"),
            Type::Stmt => write!(f, "Stmt<'a>"),
            Type::Literal => write!(f, "Literal<'a>"),
            Type::Identifier => write!(f, "Identifier<'a>"),
            Type::Type => write!(f, "Type<'a>"),
            Type::AssignmentOperator => write!(f, "AssignmentOperator"),
            Type::BinaryOperator => write!(f, "BinraryOperator"),
            Type::LogicalOperator => write!(f, "LogicalOperator"),
            Type::UnaryOperator => write!(f, "UnaryOperator"),
            Type::PostfixOperator => write!(f, "PostfixOperator"),
            Type::Argument => write!(f, "Argument<'a>"),
            Type::Optional(ty) => write!(f, "Optional<{}>", ty),
            Type::Repeated(ty) => write!(f, "Vec<{}>", ty),
            // Type::ClassMember => write!(f, "ClassMember<'a>"),
        }
    }
}

impl Type {
    pub fn string(&self, base: &str, typed: bool) -> String {
        match self {
            Type::Bool => format!("bool"),
            Type::String if typed => format!("String<'a>"),
            Type::String => format!("Cow<'a, str>"),
            Type::Expr if base == "Expr" => format!("Box<Expr<'a>>"),
            Type::Expr => format!("Expr<'a>"),
            Type::Stmt if base == "Expr" => format!("Box<Stmt<'a>>"),
            Type::Stmt if base == "Stmt" => format!("Box<Stmt<'a>>"),
            Type::Stmt => format!("Stmt<'a>"),
            Type::Identifier => format!("Identifier<'a>"),
            Type::Literal => format!("Literal<'a>"),
            Type::Type => format!("Type<'a>"),
            Type::AssignmentOperator => format!("AssignmentOperator"),
            Type::BinaryOperator => format!("BinaryOperator"),
            Type::LogicalOperator => format!("LogicalOperator"),
            Type::UnaryOperator => format!("UnaryOperator"),
            Type::PostfixOperator => format!("PostfixOperator"),
            Type::Argument => format!("Argument<'a>"),
            Type::Optional(ty) => format!("Option<{}>", ty.string(base, typed)),
            Type::Repeated(ty) => format!("Vec<{}>", ty),
            // Type::ClassMember => format!("ClassMember<'a>"),
        }
    }

    fn has_lifetime(&self) -> bool {
        match self {
            Type::Expr
            | Type::Stmt
            | Type::Literal
            | Type::Type
            | Type::Optional(_)
            | Type::Repeated(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Field(String, Type);

#[derive(Debug, Serialize, Deserialize)]
struct Node(String, Vec<Field>);

#[derive(Debug, Serialize, Deserialize)]
struct Enum(String, Vec<String>);

#[derive(Debug, Serialize)]
struct EnumCtx {
    name: String,
    variants: Vec<EnumCtxVariant>,
    lifetime: bool,
    flatten: bool,
    visitor: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct EnumCtxVariant {
    name: String,
    value: Option<String>,
    lifetime: bool,
}

#[derive(Debug, Serialize)]
struct StructCtx {
    name: String,
    lifetime: bool,
    fields: Vec<StructFieldCtx>,
}

#[derive(Debug, Serialize)]
struct StructFieldCtx {
    name: String,
    value: String,
    borrowed: bool,
}

fn generate_visitor(name: &str, nodes: &[Node]) -> EnumCtx {
    EnumCtx {
        name: name.to_owned(),
        flatten: true,
        visitor: Some(nodes.iter().map(|m| m.0.clone()).collect()),
        lifetime: true,
        variants: nodes
            .iter()
            .map(|node| {
                let lifetime = !node.1.is_empty();
                EnumCtxVariant {
                    name: node.0.clone(),
                    value: if lifetime {
                        Some(format!("{}{}<'a>", node.0, name))
                    } else {
                        Some(format!("{}{}", node.0, name))
                    },
                    lifetime: lifetime,
                }
            })
            .collect(),
    }
}

fn generate_structures(base: &str, nodes: &[Node], typed: bool) -> Vec<StructCtx> {
    nodes
        .iter()
        .map(|node| StructCtx {
            name: node.0.clone() + base,
            lifetime: !node.1.is_empty(),
            fields: node
                .1
                .iter()
                .map(|field| StructFieldCtx {
                    name: field.0.clone(),
                    value: field.1.string(base, typed),
                    borrowed: field.1.has_lifetime(),
                })
                .collect(),
        })
        .collect()
}

fn generate_enums(nodes: &[Enum]) -> Vec<EnumCtx> {
    nodes
        .iter()
        .map(|node| EnumCtx {
            name: node.0.clone(),
            lifetime: false,
            flatten: false,
            visitor: None,
            variants: node
                .1
                .iter()
                .map(|v| EnumCtxVariant {
                    name: v.clone(),
                    value: None,
                    lifetime: false,
                })
                .collect(),
        })
        .collect()
}

pub fn read_file<P: AsRef<Path>, S: serde::de::DeserializeOwned>(
    path: P,
) -> Result<S, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    Ok(ron::de::from_str(&data)?)
}

pub fn generate(
    template: &str,
    ctx: Option<Context>,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let def_path = path.join("definitions");
    let enums = read_file::<_, Vec<Enum>>(&def_path.join("enums.ron"))?;
    let expressions = read_file::<_, Vec<Node>>(&def_path.join("expressions.ron"))?;
    let stmts = read_file::<_, Vec<Node>>(&def_path.join("statements.ron"))?;

    let p = format!("{}/*", path.join("templates").to_str().unwrap());

    let mut enums_ctx = generate_enums(&enums);
    enums_ctx.push(generate_visitor("Expr", &expressions));
    enums_ctx.push(generate_visitor("Stmt", &stmts));

    let mut struct_ctx = generate_structures("Expr", &expressions, false);
    struct_ctx.extend(generate_structures("Stmt", &stmts, false));

    let templ = Tera::new(&p)?;

    let mut ctx = ctx.unwrap_or_default();
    ctx.insert("enums", &enums_ctx);
    ctx.insert("structures", &struct_ctx);

    let out = templ.render(template, &ctx)?;
    Ok(out)
}
