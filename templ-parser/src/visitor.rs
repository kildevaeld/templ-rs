use super::error::Error;
use std::borrow::Cow;
use std::collections::HashMap;
use templ_ast::*;

pub type VisitorResult = Result<(), Error>;

#[derive(Debug, Clone)]
pub struct Visitor<'a> {
    templates: HashMap<String, TemplateStmt<'a>>,
}

impl<'a> Visitor<'a> {
    pub fn new() -> Visitor<'a> {
        Visitor {
            templates: HashMap::default(),
        }
    }

    pub fn visit(mut self, module: &mut ModuleStmt<'a>) -> VisitorResult {
        self.visit_module_stmt(module)?;
        Ok(())
    }
}

impl<'a> ExprVisitor<'a, VisitorResult> for Visitor<'a> {
    fn visit_lookup_expr(&mut self, _e: &mut LookupExpr<'a>) -> VisitorResult {
        Ok(())
    }
    fn visit_literal_expr(&mut self, _e: &mut LiteralExpr<'a>) -> VisitorResult {
        Ok(())
    }
    fn visit_call_expr(&mut self, e: &mut CallExpr<'a>) -> VisitorResult {
        unimplemented!("visit_call_expr({:?})", e);
    }
    fn visit_member_expr(&mut self, e: &mut MemberExpr<'a>) -> VisitorResult {
        e.object.accept(self)?;
        e.property.accept(self)?;
        Ok(())
    }
    fn visit_index_expr(&mut self, e: &mut IndexExpr<'a>) -> VisitorResult {
        unimplemented!("visit_index_expr({:?})", e);
    }
    fn visit_filter_expr(&mut self, _e: &mut FilterExpr<'a>) -> VisitorResult {
        // unimplemented!("visit_filter_expr({:?})", e);
        Ok(())
    }
    fn visit_group_expr(&mut self, e: &mut GroupExpr<'a>) -> VisitorResult {
        unimplemented!("visit_group_expr({:?})", e);
    }
    fn visit_binary_expr(&mut self, _e: &mut BinaryExpr<'a>) -> VisitorResult {
        Ok(())
    }
    fn visit_logical_expr(&mut self, e: &mut LogicalExpr<'a>) -> VisitorResult {
        unimplemented!("visit_logical_expr({:?})", e);
    }
    fn visit_postfix_expr(&mut self, e: &mut PostfixExpr<'a>) -> VisitorResult {
        unimplemented!("visit_postfix_expr({:?})", e);
    }
    fn visit_unary_expr(&mut self, e: &mut UnaryExpr<'a>) -> VisitorResult {
        unimplemented!("visit_unary_expr({:?})", e);
    }
}

fn get_string<'a>(found: &'a TagStmt<'a>) -> &'a Cow<'a, str> {
    let args = match &found.argument {
        Some(s) => s,
        None => {
            panic!("should provide name");
        }
    };

    let name = match args {
        Expr::Literal(l) => match &l.value {
            Literal::String(s) => s,
            _ => {
                //
                panic!("block argument should be a string")
            }
        },
        Expr::Lookup(i) => &i.value.value,
        _ => {
            //
            panic!("block argument should be a string")
        }
    };

    name
}

impl<'a> StmtVisitor<'a, VisitorResult> for Visitor<'a> {
    fn visit_module_stmt(&mut self, e: &mut ModuleStmt<'a>) -> VisitorResult {
        for m in e.statements.iter_mut() {
            m.accept(self)?;
        }

        Ok(())
    }
    fn visit_template_stmt(&mut self, e: &mut TemplateStmt<'a>) -> VisitorResult {
        if self.templates.get(e.name.as_ref()).is_some() {
            panic!("template with that name already exists");
        }

        for p in e.params.iter_mut() {
            p.accept(self)?;
        }

        e.body.accept(self)?;

        if let Some(parent) = &mut e.extends {
            let parent = match self.templates.get(parent.value.as_ref()) {
                Some(s) => s,
                None => {
                    panic!("no template with that name");
                }
            };

            // for p in parent.params.iter() {
            //     if e.params.iter().find(|m| *m == p).is_none() {
            //         e.params.push(p.clone());
            //     }
            // }

            let (parent_body, body) = match (parent.body.as_ref(), e.body.as_ref()) {
                (Stmt::Block(p), Stmt::Block(b)) => (p, b),
                _ => panic!("not body"),
            };

            let body = parent_body
                .value
                .iter()
                .map(|s| {
                    if let Stmt::Tag(tag) = s {
                        let parent_name = get_string(tag);
                        if let Some(found) = body.value.iter().find(|i| match i {
                            Stmt::Tag(t) if t.name.value == "block" => true,
                            _ => false,
                        }) {
                            let found: &TagStmt = match found {
                                Stmt::Tag(t) => t,
                                _ => panic!("should not happen"),
                            };

                            let name = get_string(found);

                            if name == parent_name {
                                return Stmt::Tag(found.clone());
                            }
                        }
                    }
                    s.clone()
                })
                .collect();
            e.body = Box::new(Stmt::Block(BlockStmt::new(body)));
        }

        self.templates.insert(e.name.as_ref().to_owned(), e.clone());

        Ok(())
    }
    fn visit_block_stmt(&mut self, e: &mut BlockStmt<'a>) -> VisitorResult {
        for v in e.value.iter_mut() {
            v.accept(self)?;
        }
        Ok(())
    }
    fn visit_param_stmt(&mut self, _e: &mut ParamStmt<'a>) -> VisitorResult {
        Ok(())
    }
    fn visit_raw_stmt(&mut self, _e: &mut RawStmt<'a>) -> VisitorResult {
        Ok(())
    }
    fn visit_model_stmt(&mut self, _e: &mut ModelStmt<'a>) -> VisitorResult {
        Ok(())
    }
    fn visit_expr_stmt(&mut self, e: &mut ExprStmt<'a>) -> VisitorResult {
        e.value.accept(self)
    }
    fn visit_loop_stmt(&mut self, e: &mut LoopStmt<'a>) -> VisitorResult {
        unimplemented!("visit_loop_stmt({:?})", e);
    }
    fn visit_comment_stmt(&mut self, e: &mut CommentStmt<'a>) -> VisitorResult {
        unimplemented!("visit_comment_stmt({:?})", e);
    }
    fn visit_tag_stmt(&mut self, _e: &mut TagStmt<'a>) -> VisitorResult {
        Ok(())
        //unimplemented!("visit_tag_stmt({:?})", e);
    }
    fn visit_if_stmt(&mut self, e: &mut IfStmt<'a>) -> VisitorResult {
        e.condition.accept(self)?;
        e.consequence.accept(self)?;
        if let Some(a) = &mut e.alternative {
            a.accept(self)?;
        }
        Ok(())
    }
    fn visit_elif_stmt(&mut self, e: &mut ElifStmt<'a>) -> VisitorResult {
        e.condition.accept(self)?;
        e.consequence.accept(self)?;
        if let Some(a) = &mut e.alternative {
            a.accept(self)?;
        }
        Ok(())
    }
    fn visit_else_stmt(&mut self, e: &mut ElseStmt<'a>) -> VisitorResult {
        e.block.accept(self)
    }
}
