use super::chunk::{ChunkBuilder, ChunkIndex};
use super::constant::Constant;
use super::error::CompileError;
use super::opcode::OpCode;
use super::parse::parse;
use crate::template::Template;
use std::collections::HashMap;
use std::ops::DerefMut;
use templ_ast::*;
use templ_runtime::Value;
use templ_runtime::{Number as ValueNumber, Runtime, Type as RuntimeType};

pub type VisitorResult = Result<(), CompileError>;

#[derive(Debug, Clone)]
pub(crate) struct Local {
    pub(crate) depth: i32,
    pub(crate) name: String,
    pub(crate) is_upvalue: bool,
}

impl Local {
    pub fn new(depth: i32, name: String, is_upvalue: bool) -> Local {
        Local {
            depth,
            name,
            is_upvalue,
        }
    }
}

#[derive(Default, Debug)]
struct TemplateState {
    params: Vec<(String, RuntimeType)>,
    locals: Vec<Local>,
    scope_depth: i32,
    chunk: ChunkBuilder,
    member_depth: i32,
}

impl TemplateState {
    pub fn chunk(&self) -> &ChunkBuilder {
        &self.chunk
    }

    pub fn chunk_mut(&mut self) -> &mut ChunkBuilder {
        &mut self.chunk
    }

    pub fn add_local(&mut self, name: String) {
        //if self.locals.len() > 256 {}
        self.locals.push(Local::new(-1, name, false))
    }

    pub fn pop_locals(&mut self) {
        while !self.locals.is_empty()
            && self.locals.last().map(|n| n.depth).unwrap_or(0) > self.scope_depth
        {
            // if self.locals[self.locals.len() - 1].is_upvalue {
            //     self.chunk_mut().emit(OpCode::CloseUpValue);
            // } else {
            //     self.chunk_mut().emit(OpCode::Pop);
            // }
            self.chunk_mut().emit(OpCode::Pop);
            self.locals.pop();
        }
    }

    pub fn is_local(&self) -> bool {
        self.scope_depth > 0
    }

    pub fn resolve_local(&self, name: String) -> Option<usize> {
        match self.locals.iter().enumerate().find(|m| (m.1).name == name) {
            Some(m) => Some(m.0),
            None => None,
        }
    }

    pub fn mark_initialized(&mut self) {
        let scope = self.scope_depth;
        if let Some(last) = self.locals.last_mut() {
            last.depth = scope;
        }
    }

    pub fn make_constant(&mut self, constant: Value) -> ChunkIndex {
        self.chunk_mut().make_constant(constant)
    }

    pub fn declare_variable(&mut self, name: String) -> Result<(), CompileError> {
        // if self.scope_depth == 0 {
        //     return Ok(());
        // }
        for i in (0..self.locals.len()).into_iter().rev() {
            if self.locals[i].depth == -1 && self.locals[i].depth < self.scope_depth {
                break;
            }
            if self.locals[i].name == name {
                println!("NAME {}", name);
                return Err(CompileError::DuplicateVariable);
            }
        }

        self.add_local(name);

        Ok(())
    }

    pub fn define_variable(&mut self, global: usize) {
        self.mark_initialized();
        // if self.scope_depth > 0 {
        //     self.mark_initialized();
        //     return;
        // }

        // self.chunk_mut()
        //     .emit_bytes(OpCode::DefineGlobal, global as u8);
    }

    pub fn parse_var(&mut self, name: String) -> Result<usize, CompileError> {
        self.declare_variable(name.clone())?;
        if self.scope_depth > 0 {
            Ok(0)
        } else {
            Ok(self.chunk_mut().make_constant(Value::String(name)))
        }
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self, pop_locals: bool) {
        self.scope_depth -= 1;

        if pop_locals {
            self.pop_locals();
        }
    }

    pub fn emit<BS: Into<u8>>(&mut self, code: BS) {
        self.chunk_mut().emit(code)
    }

    pub fn emit_bytes<BS: Into<u8>, BS2: Into<u8>>(&mut self, code: BS, byte: BS2) {
        self.chunk_mut().emit(code);
        self.chunk_mut().emit(byte);
    }

    pub fn variable(&mut self, name: String) {
        // let (a, get, set) = if let Some(a) = self.resolve_local(name) {
        //     (a, OpCode::GetLocal, OpCode::SetLocal)
        // } else if let Some(a) = self.resolve_upvalue(mc, name) {
        //     (a, OpCode::GetUpValue, OpCode::SetUpValue)
        // } else {
        //     let a = self.chunk_mut().make_constant(Constant::String(name));
        //     (a, OpCode::GetGlobal, OpCode::SetGlobal)
        // };

        // if self.assign {
        //     self.emit_bytes(set, a as u8)
        // } else {
        //     self.emit_bytes(get, a as u8);
        // }
        let (a, get) = if let Some(a) = self.resolve_local(name.clone()) {
            (a, OpCode::GetLocal)
        } else {
            let a = self.chunk_mut().make_constant(Value::String(name));
            (a, OpCode::GetGlobal)
        };

        self.emit_bytes(get, a as u8);
    }
}

pub struct Visitor {
    runtime: Runtime,
    types: HashMap<String, RuntimeType>,
    current_model: Option<(String, Vec<(String, RuntimeType)>)>,
    current_template: Option<TemplateState>,
    templates: Vec<Template>,
}

impl Visitor {
    pub fn new(runtime: Runtime) -> Visitor {
        Visitor {
            runtime,
            types: HashMap::default(),
            current_model: None,
            current_template: None,
            templates: Vec::default(),
        }
    }

    pub fn compile<'a>(mut self, ast: &mut ModuleStmt<'a>) -> Result<Vec<Template>, CompileError> {
        self.visit_module_stmt(ast)?;
        Ok(self.templates)
    }

    fn chunk_mut(&mut self) -> &mut ChunkBuilder {
        &mut self.current_template.as_mut().unwrap().chunk
    }

    fn template_mut(&mut self) -> &mut TemplateState {
        self.current_template.as_mut().unwrap()
    }

    // pub fn visit<'a>(&mut self, module: &mut ModuleStmt<'a>) -> VisitorResult {
    //     self.visit_module_stmt(module)
    // }
}

impl<'a> ExprVisitor<'a, VisitorResult> for Visitor {
    fn visit_lookup_expr(&mut self, e: &mut LookupExpr<'a>) -> VisitorResult {
        if self.template_mut().member_depth > 0 {
            let global = self
                .template_mut()
                .chunk_mut()
                .make_constant(Value::String(e.value.value.as_ref().to_owned()));
            // if self.template_mut().assign {
            //     self.template_mut()
            //         .emit_bytes(OpCode::SetProperty, global as u8);
            // } else {
            //     self.template_mut()
            //         .emit_bytes(OpCode::GetProperty, global as u8);
            // }
            self.template_mut()
                .emit_bytes(OpCode::GetProperty, global as u8);
        } else {
            self.template_mut()
                .variable(e.value.value.as_ref().to_owned());
        }

        Ok(())
    }
    fn visit_literal_expr(&mut self, e: &mut LiteralExpr<'a>) -> VisitorResult {
        let val = match &mut e.value {
            Literal::Number(Number::Double(d)) => Value::Number(ValueNumber::Float(*d)),
            Literal::Number(Number::Integer(i)) => Value::Number(ValueNumber::Integer(*i)),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::String(s) => Value::String(s.as_ref().to_owned()),
            Literal::Slice(v) => {
                let len = v.len();
                for i in v {
                    i.accept(self)?;
                }
                self.chunk_mut().emit(OpCode::Slice);
                self.chunk_mut().emit16(len as u16);
                return Ok(());
            }
            Literal::Map(v) => {
                let len = v.len() * 2;
                for (k, v) in v {
                    self.chunk_mut()
                        .emit_constant(Value::String(k.as_ref().to_owned()));
                    v.accept(self)?;
                }
                self.chunk_mut().emit(OpCode::Map);
                self.chunk_mut().emit16(len as u16);
                return Ok(());
            }
            _ => unimplemented!("literal {:?}", e),
        };
        self.chunk_mut().emit_constant(val);
        Ok(())
    }
    fn visit_call_expr(&mut self, e: &mut CallExpr<'a>) -> VisitorResult {
        unimplemented!("visit_call_expr({:?})", e);
    }
    fn visit_member_expr(&mut self, e: &mut MemberExpr<'a>) -> VisitorResult {
        e.object.accept(self)?;

        self.template_mut().member_depth += 1;

        e.property.accept(self)?;

        self.template_mut().member_depth -= 1;

        Ok(())
    }
    fn visit_index_expr(&mut self, e: &mut IndexExpr<'a>) -> VisitorResult {
        unimplemented!("visit_index_expr({:?})", e);
    }
    fn visit_filter_expr(&mut self, e: &mut FilterExpr<'a>) -> VisitorResult {
        match e.filter.as_ref() {
            Expr::Lookup(l) => {
                //
                let filter = match self.runtime.filter(l.value.value.as_ref()) {
                    Some(filter) => filter.clone(),
                    None => {
                        unimplemented!("not found {:?}", l);
                    }
                };
                self.chunk_mut().emit_constant(Value::Filter(filter));
            }
            _ => {
                e.filter.accept(self)?;
            }
        }

        e.object.accept(self)?;

        self.chunk_mut().emit(OpCode::Call1);

        Ok(())
        //unimplemented!("visit_filter_expr({:?})", e);
    }
    fn visit_group_expr(&mut self, e: &mut GroupExpr<'a>) -> VisitorResult {
        unimplemented!("visit_group_expr({:?})", e);
    }
    fn visit_binary_expr(&mut self, e: &mut BinaryExpr<'a>) -> VisitorResult {
        e.left.accept(self)?;
        e.right.accept(self)?;

        let opc = match &e.op {
            BinaryOperator::Add => OpCode::Add,
            BinaryOperator::Sub => OpCode::Substract,
            BinaryOperator::Mul => OpCode::Multiply,
            BinaryOperator::Mod => OpCode::Modulo,
            BinaryOperator::Div => OpCode::Divide,
            BinaryOperator::Eq => OpCode::Equal,
            BinaryOperator::Lte => {
                self.template_mut().emit(OpCode::Greater);
                OpCode::Not
            }
            BinaryOperator::Lt => OpCode::Less,
            BinaryOperator::Gt => OpCode::Greater,
            BinaryOperator::Gte => {
                self.template_mut().emit(OpCode::Less);
                OpCode::Not
            }
            BinaryOperator::Neq => {
                self.template_mut().emit(OpCode::Equal);
                OpCode::Not
            }
            //BinaryOperator::Is => OpCode::InstanceOf,
            _ => unimplemented!("binary {:?}", e.op),
        };

        self.template_mut().emit(opc);

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

impl<'a> StmtVisitor<'a, VisitorResult> for Visitor {
    fn visit_module_stmt(&mut self, e: &mut ModuleStmt<'a>) -> VisitorResult {
        for stmt in e.statements.iter_mut() {
            stmt.accept(self)?;
        }
        Ok(())
    }
    fn visit_template_stmt(&mut self, e: &mut TemplateStmt<'a>) -> VisitorResult {
        self.current_template = Some(TemplateState::default());

        for p in e.params.iter_mut() {
            p.accept(self)?;
        }

        e.body.accept(self)?;

        let template = self.current_template.take().unwrap();

        let template = Template {
            name: e.name.as_ref().to_owned(),
            chunk: template.chunk.build(),
            params: template.params.into_iter().map(|m| m.1).collect(),
        };

        self.templates.push(template);

        Ok(())
    }
    fn visit_block_stmt(&mut self, e: &mut BlockStmt<'a>) -> VisitorResult {
        for m in e.value.iter_mut() {
            m.accept(self)?;
        }

        Ok(())
    }
    fn visit_param_stmt(&mut self, e: &mut ParamStmt<'a>) -> VisitorResult {
        if let Some(model) = self.current_model.as_mut() {
            model.1.push((
                e.name.as_ref().to_owned(),
                ast_type_to_type(&self.types, &e.ty)?,
            ));
        } else if let Some(template) = self.current_template.as_mut() {
            template.params.push((
                e.name.as_ref().to_owned(),
                ast_type_to_type(&self.types, &e.ty)?,
            ));
            let global = template.parse_var(e.name.as_ref().to_owned())?;
            template.define_variable(global);
        }
        Ok(())
    }
    fn visit_raw_stmt(&mut self, e: &mut RawStmt<'a>) -> VisitorResult {
        self.template_mut()
            .chunk_mut()
            .emit_constant(Value::String(e.value.as_ref().to_owned()));

        self.chunk_mut().emit(OpCode::Render0);

        Ok(())
    }
    fn visit_model_stmt(&mut self, e: &mut ModelStmt<'a>) -> VisitorResult {
        self.current_model = Some((e.name.value.as_ref().to_owned(), Vec::new()));
        for p in e.fields.iter_mut() {
            p.accept(self)?;
        }
        let model = self.current_model.take().unwrap();
        self.types.insert(model.0, RuntimeType::Struct(model.1));

        Ok(())
    }
    fn visit_expr_stmt(&mut self, e: &mut ExprStmt<'a>) -> VisitorResult {
        e.value.accept(self)?;
        self.chunk_mut().emit(OpCode::Render0);
        Ok(())
    }
    fn visit_loop_stmt(&mut self, e: &mut LoopStmt<'a>) -> VisitorResult {
        unimplemented!("visit_loop_stmt({:?})", e);
    }
    fn visit_comment_stmt(&mut self, e: &mut CommentStmt<'a>) -> VisitorResult {
        unimplemented!("visit_comment_stmt({:?})", e);
    }
    fn visit_tag_stmt(&mut self, e: &mut TagStmt<'a>) -> VisitorResult {
        if e.name.value.as_ref() == "block" {
            if let Some(block) = &mut e.block {
                block.accept(self)?;
            }
        } else {
            let name = e.name.value.as_ref();
            let block = match self.runtime.block(name) {
                Some(s) => s.clone(),
                None => {
                    panic!("could not find block");
                }
            };

            self.template_mut().make_constant(Value::Block(block));

            for a in e.argument.iter_mut() {
                a.accept(self)?;
            }

            for a in e.block.iter_mut() {
                a.accept(self)?;
            }

            let o = if e.argument.is_some() {
                if e.block.is_some() {
                    OpCode::Render2
                } else {
                    OpCode::Render1
                }
            } else {
                if e.block.is_some() {
                    OpCode::Render1
                } else {
                    OpCode::Render0
                }
            };

            self.chunk_mut().emit(o);
        }

        Ok(())
    }
    fn visit_if_stmt(&mut self, e: &mut IfStmt<'a>) -> VisitorResult {
        e.condition.accept(self)?;
        let elsejump = self
            .template_mut()
            .chunk_mut()
            .emit_jump(OpCode::JumpIfFalse);

        self.template_mut().emit(OpCode::Pop);
        e.consequence.accept(self)?;

        let end = self.chunk_mut().emit_jump(OpCode::Jump);

        self.template_mut().chunk_mut().patch_jump(elsejump);
        self.template_mut().emit(OpCode::Pop);

        if e.alternative.is_some() {
            e.alternative.as_mut().unwrap().accept(self)?;
        }

        self.template_mut().chunk_mut().patch_jump(end);

        Ok(())
    }
    fn visit_elif_stmt(&mut self, e: &mut ElifStmt<'a>) -> VisitorResult {
        e.condition.accept(self)?;
        let elsejump = self
            .template_mut()
            .chunk_mut()
            .emit_jump(OpCode::JumpIfFalse);

        self.template_mut().emit(OpCode::Pop);
        e.consequence.accept(self)?;

        let end = self.chunk_mut().emit_jump(OpCode::Jump);

        self.template_mut().chunk_mut().patch_jump(elsejump);
        self.template_mut().emit(OpCode::Pop);

        if e.alternative.is_some() {
            e.alternative.as_mut().unwrap().accept(self)?;
        }

        self.template_mut().chunk_mut().patch_jump(end);

        Ok(())
    }
    fn visit_else_stmt(&mut self, e: &mut ElseStmt<'a>) -> VisitorResult {
        e.block.accept(self)
    }
}

fn ast_type_to_type(
    types: &HashMap<String, RuntimeType>,
    t: &templ_ast::Type<'_>,
) -> Result<RuntimeType, CompileError> {
    let o = match t {
        Type::Bool => RuntimeType::Bool,
        Type::Date => RuntimeType::Date,
        Type::Number => RuntimeType::Float,
        Type::Slice(s) => RuntimeType::Slice(Box::new(ast_type_to_type(types, s)?)),
        Type::String => RuntimeType::String,
        Type::User(name) => match types.get(*name) {
            Some(s) => s.clone(),
            None => panic!(""),
        },
    };

    Ok(o)
}
