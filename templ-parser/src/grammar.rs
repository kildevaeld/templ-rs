use peg::ParseLiteral;
use std::borrow::Cow;
use std::collections::HashMap;
use templ_ast::*;

pub(crate) fn resolve_member<'a>(l: Expr<'a>, mut o: Vec<Expr<'a>>) -> Expr<'a> {
    if o.is_empty() {
        return l;
    }
    let first = o.pop().unwrap();
    let left = Expr::Member(MemberExpr::new(Box::new(l), Box::new(first)));
    if o.is_empty() {
        return left;
    }
    return resolve_member(left, o);
}

peg::parser! {

    pub(crate) grammar templ_parser() for str {
        pub rule parse() -> ModuleStmt<'input>
            = __ s:statements() __ ![_] { ModuleStmt::new(s) }

        rule statements() -> Vec<Stmt<'input>>
            = s:statement() ** __() {
                s
            }

        rule statement() -> Stmt<'input>
            = s:( template_stmt() / model_stmt() ) { s }



        rule model_stmt() -> Stmt<'input>
            = type_token() _  n:identifier() _ e:("extends" _ e:identifier() { e })? __  "{" __ p:newlinesep(<param_stmt()>) __ "}" {
                Stmt::Model(ModelStmt::new(n, p, e))
            }

        rule template_stmt() -> Stmt<'input>
            = h:open_close(<template_header()>) line_terminator_sequence()  c:block_stmt() open_close(<end_token()>) {
                Stmt::Template(TemplateStmt::new(h.0.into(), h.1, Box::new(c), h.2))
            }

        rule template_header() -> (&'input str, Vec<Stmt<'input>>, Option<Identifier<'input>>)
            = template_token() _ i:$identifier()  _ "(" _ p: commasep(<param_stmt()>)  _ ")" e:(_ "extends" _ e:identifier() { e })? { (i, p, e)  }

        rule param_stmt() -> Stmt<'input>
            = n:$identifier() whitespace()+ t:types() { Stmt::Param(ParamStmt::new(n.into(), t)) }


        rule block_stmt() -> Stmt<'input>
            = b:(
                expr_stmt()
                / content_stmt()
                / loop_stmt()
                / condition_stmt()
                / comment_stmt()
                / tag_stmt()
            )* {
                Stmt::Block(BlockStmt::new(b))
            }

        rule comment_stmt() -> Stmt<'input>
            = "{#" c:$( ( !"#}" a:( [_] ) { a } )+ ) "#}" {
                Stmt::Comment(CommentStmt::new(c.into()))
            }


        rule tag_stmt() -> Stmt<'input>
            = open() _ i:identifier() _ e:expression() _ close() b:( __ b:block_stmt() __ open() _ c:identifier() _ close() {?
                if !c.value.starts_with("end") || &c.value[3..c.value.len()] != i.value {
                    Err("not ender")
                } else {
                    Ok(b)
                }
            } )? {
                Stmt::Tag(TagStmt::new(i, Some(e), b.map(Box::new)))
            }


        rule condition_stmt() -> Stmt<'input>
            = open() _ if_token()  e:expression() _ close() __ b:block_stmt() __ a:condition_alternative()? __ open_close(<endif_token()>) {
                Stmt::If(IfStmt::new(e, Box::new(b), a))
            }

        rule condition_alternative() -> Box<Stmt<'input>>
            = e:( elif_stmt() / else_stmt()) { Box::new(e) }

        rule elif_stmt() -> Stmt<'input>
            =  open() _ elif_token() _  e:expression() _ close() __ b:block_stmt() __ a:condition_alternative()? {
                Stmt::Elif(ElifStmt::new(e, Box::new(b), a))
            }

        rule else_stmt() -> Stmt<'input>
            = open() _ else_token() _ close() __ b:block_stmt() {
                Stmt::Else(ElseStmt::new(Box::new(b)))
            }

        rule loop_stmt() -> Stmt<'input>
            = h:open_close(<loop_header()>) b:block_stmt() open_close(<endfor_token()>) {
                Stmt::Loop(LoopStmt::new(h.0, h.1, h.2, Box::new(b)))
            }

        rule loop_header() -> (Identifier<'input>, Option<Identifier<'input>>,Expr<'input>)
            = for_token() _ k:identifier() _ v:("," _ v:identifier() { v})? _ in_token() _ e:expression() { (k, v, e) }

        rule content_stmt() -> Stmt<'input>
            = t:$((!( open() / close() / comment_stmt() / "{{" / "}}" ) r:$source())+) { Stmt::Raw(RawStmt::new(t.into())) }


        rule expr_stmt() -> Stmt<'input>
            = "{{" __ e:expression() __ "}}" { Stmt::Expr(ExprStmt::new(e)) }

        // Expressions
        rule expression() -> Expr<'input>
            = precedence! {
                lhs:(@) _ "|" !"|" _ rhs:(identifier_expr()) { Expr::Filter(FilterExpr::new(Box::new(lhs), Box::new(rhs))) }
                lhs:(@) _ "&&" _ rhs:@ { Expr::Logical(LogicalExpr::new(Box::new(lhs), Box::new(rhs), LogicalOperator::And)) }
                lhs:(@) _ "||" _ rhs:@ { Expr::Logical(LogicalExpr::new(Box::new(lhs), Box::new(rhs), LogicalOperator::Or)) }
                --
                // lhs:(@) _ "&" !("&" / "=") _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::BitwiseAnd)) }
                // lhs:(@) _ "|" !("|" / "=") _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::BitwiseOr)) }
                // lhs:(@) _ "^" !( "=" ) _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::BitwiseXor)) }
                // --
                lhs:(@) _ "==" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Eq)) }
                lhs:(@) _ "!=" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Neq)) }
                --
                lhs:(@) _ "<=" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Lte)) }
                lhs:(@) _ ">=" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Gte)) }
                lhs:(@) _ "<" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Lt)) }
                lhs:(@) _ ">" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Gt)) }
                lhs:(@) _ "is" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Is)) }
                --
                // lhs:(@) _ "<<" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::ShiftLeft)) }
                // lhs:(@) _ ">>" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::ShiftRight)) }
                // --
                lhs:(@) _ "+" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Add)) }
                lhs:(@) _ "-" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Sub)) }
                --
                lhs:(@) _ "*" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Mul)) }
                lhs:(@) _ "/" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Div)) }
                lhs:(@) _ "%" _ rhs:@ { Expr::Binary(BinaryExpr::new(Box::new(lhs), Box::new(rhs), BinaryOperator::Mod)) }
                --
                op:unary_operator() rhs:@ { Expr::Unary(UnaryExpr::new(Box::new(rhs), op)) }
                lhs:@ op:postfix_operator() { Expr::Postfix(PostfixExpr::new(Box::new(lhs),op)) }
                --
                lhs:(@) _ a:arguments() { Expr::Call(CallExpr::new(Box::new(lhs), a)) }
                lhs:(@) "." e:@ { Expr::Member(MemberExpr::new(Box::new(lhs), Box::new(e))) }
                lhs:(@) _ "[" __ e:expression() __ "]" { Expr::Index(IndexExpr::new(Box::new(lhs), Box::new(e)))  }
                __
                p:primary_expr() { p }
            }


        rule arguments() -> Vec<Expr<'input>>
            = "(" __()  a:((expression()) ** ( __() "," __() )) __() ")" { a }



        rule member_expr() -> Expr<'input>
            = i:index_expr() { i }
                / o:( primary_expr()  )
                p:( "." i:identifier_expr() { i } )* {
                    resolve_member(o, p)
            }

        rule primary_expr() -> Expr<'input>
            =  lit:literal_expr() { lit }
            / i:identifier() { Expr::Lookup(LookupExpr::new(i)) }
            / "(" e:expression() ")" { Expr::Group(GroupExpr::new(Box::new(e))) }

        rule index_expr() -> Expr<'input>
            = e:primary_expr() "[" __ i:expression() __ "]" {
                Expr::Index(IndexExpr::new(Box::new(e), Box::new(i)))
            }

        rule identifier_expr() -> Expr<'input>
            = i:identifier() { Expr::Lookup(LookupExpr::new(i)) }

        // Literals

        rule literal_expr() -> Expr<'input>
            = lit:(
                literal_slice()
                / literal_number()
                / literal_boolean()
                / literal_string()
                / literal_map()
            ) { Expr::Literal(LiteralExpr::new(lit)) }


        rule literal_boolean() -> Literal<'input>
            = b:$("true" / "false") {
                Literal::Bool(if b == "true" { true } else { false })
        }

        /** Array literal */
        rule literal_slice() -> Literal<'input>
            = "[" __ i:array_items() __ "]" {
                Literal::Slice(i)
            }

        rule array_items() -> Vec<Expr<'input>>
            = i:expression() ** (__() "," __()) {
                i
            }


        /**  Map Literal */
        rule literal_map() -> Literal<'input>
            = "{" __  p:map_prop() ** ( __ "," __) __  "}" {
                let mut map = HashMap::default();
                for prop in p.into_iter() {
                    map.insert(prop.0.into(), prop.1);
                }
                Literal::Map(map)
            }

        rule map_prop() -> (Cow<'input, str>, Expr<'input>)
            = k:( l:literal_string() { l.as_str().unwrap().clone() }  / i:identifier() { i.value.clone() }) _ ":" _ v:expression() {
                (k, v)
            }
         /**  Number Literals  */
         rule literal_number() -> Literal<'input>
            =  n:(double() / int()) { Literal::Number(n) }

        rule int() -> Number
            = i:$("0" / ['1'..='9'] ['0'..='9']*)  {
                Number::Integer(i.parse::<f64>().unwrap())
            }

        rule double() -> Number
            =  i:$(("0" / ['1'..='9'] ['0'..='9']*) "." ['0'..='9']+) {
                Number::Double(i.parse().unwrap())
            }

        /**  String Literals */
        rule literal_string() -> Literal<'input>
            = "\"" s:$string_literal() "\"" { Literal::String(s.into()) }

        rule string_literal()
            = (raw_string() / escape())*

        rule raw_string()
            =  (!("\\" / "\"") source() )+

        rule hex()
            =  ['0'..='9'] / ['a'..='f'] / ['A'..='F']

        rule unicode_hex()
            =  hex()*<1, 6>

        rule predefined()
            =  "n" / "r" / "t" / "\\" / "0" / "\"" / "'"

        rule byte()
            =  "x"  hex()*<2>

        rule unicode()
            =  "u" "{" unicode_hex() "}"

        rule escape()
            = "\\" (predefined() / byte() / unicode())



        // Types

        rule types() -> Type<'input>
            = keyword("date") { Type::Date }
            / keyword("string")  { Type::String }
            / keyword("bool") { Type::Bool }
            / keyword("number") { Type::Number }
            / "[]" _ t:types() { Type::Slice(Box::new(t)) }
            / i:$identifier() { Type::User(i) }



        rule template_token() = keyword("template")

        rule type_token() = keyword("type")

        rule for_token() = keyword("for")

        rule end_token() -> &'input str = k:$keyword("end") { k }

        rule endif_token() -> &'input str = k:$keyword("endif") { k }

        rule endfor_token() -> &'input str = k:$keyword("endfor") { k }

        rule in_token() = keyword("in")

        rule if_token() = keyword("if")

        rule elif_token() = keyword("elif")

        rule else_token() = keyword("else")

        rule keyswords() = end_token() / endif_token() / endfor_token() / in_token() / elif_token() / else_token()


        /* *** Operators *** */
        rule unary_operator() -> UnaryOperator
            = "++" { UnaryOperator::Increment }
            / "--" { UnaryOperator::Decrement }
            / "+" !"=" { UnaryOperator::Plus }
            / "-" !"=" { UnaryOperator::Minus }
            / "!" !"=" { UnaryOperator::Not }

        rule postfix_operator() -> PostfixOperator
            = "++" { PostfixOperator::Increment }
            / "--" { PostfixOperator::Decrement }

        rule open() = "{%"
        rule close() = "%}"

        rule open_close<T>(x: rule<T>) -> T
            = open() _ v:x() _ close() { v }

        rule commasep<T>(x: rule<T>) -> Vec<T>
            = v:(x() ** ("," _))  {v}

        rule newlinesep<T>(x: rule<T>) -> Vec<T>
            = v:(x() ** (line_terminator_sequence() __) ) __ {v}

        rule keyword(id: &'static str) = ##parse_string_literal(id) !['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']

        rule identifier() -> Identifier<'input>
            = s:position!() i:$identifier_raw() e:position!() { Identifier::new(Location(s,e), i) }

        rule identifier_raw()
            = !( keyswords() )  ['a'..='z' | 'A'..='Z' | '_'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*

        rule source() = [_]


        rule whitespace()
            =  "\t" / "\\v" / "\\f" / " " / "\u{00A0}" / "\u{FEFF}"

        rule line_terminator()
            =  "\n" / "\r" / "\u{2028}" / "\u{2029}"

        rule line_terminator_sequence()
            =  "\n" / "\r\n" / "\r" / "\u{2028}" / "\u{2029}"

        rule _() = whitespace()*

        rule __() = (whitespace() / line_terminator_sequence() )*
    }
}
