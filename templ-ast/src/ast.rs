/*! WARNING: AUTO GENERATED  - DO NOT EDIT **/

use super::types::*;
use std::borrow::Cow;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};






pub trait ExprVisitor<'a, R> {
    
    fn visit_lookup_expr(&mut self, e:&mut LookupExpr<'a>) -> R;
    
    fn visit_literal_expr(&mut self, e:&mut LiteralExpr<'a>) -> R;
    
    fn visit_call_expr(&mut self, e:&mut CallExpr<'a>) -> R;
    
    fn visit_member_expr(&mut self, e:&mut MemberExpr<'a>) -> R;
    
    fn visit_index_expr(&mut self, e:&mut IndexExpr<'a>) -> R;
    
    fn visit_filter_expr(&mut self, e:&mut FilterExpr<'a>) -> R;
    
    fn visit_group_expr(&mut self, e:&mut GroupExpr<'a>) -> R;
    
    fn visit_binary_expr(&mut self, e:&mut BinaryExpr<'a>) -> R;
    
    fn visit_logical_expr(&mut self, e:&mut LogicalExpr<'a>) -> R;
    
    fn visit_postfix_expr(&mut self, e:&mut PostfixExpr<'a>) -> R;
    
    fn visit_unary_expr(&mut self, e:&mut UnaryExpr<'a>) -> R;
    
}

pub trait StmtVisitor<'a, R> {
    
    fn visit_module_stmt(&mut self, e:&mut ModuleStmt<'a>) -> R;
    
    fn visit_template_stmt(&mut self, e:&mut TemplateStmt<'a>) -> R;
    
    fn visit_block_stmt(&mut self, e:&mut BlockStmt<'a>) -> R;
    
    fn visit_param_stmt(&mut self, e:&mut ParamStmt<'a>) -> R;
    
    fn visit_raw_stmt(&mut self, e:&mut RawStmt<'a>) -> R;
    
    fn visit_model_stmt(&mut self, e:&mut ModelStmt<'a>) -> R;
    
    fn visit_expr_stmt(&mut self, e:&mut ExprStmt<'a>) -> R;
    
    fn visit_loop_stmt(&mut self, e:&mut LoopStmt<'a>) -> R;
    
    fn visit_comment_stmt(&mut self, e:&mut CommentStmt<'a>) -> R;
    
    fn visit_tag_stmt(&mut self, e:&mut TagStmt<'a>) -> R;
    
    fn visit_if_stmt(&mut self, e:&mut IfStmt<'a>) -> R;
    
    fn visit_elif_stmt(&mut self, e:&mut ElifStmt<'a>) -> R;
    
    fn visit_else_stmt(&mut self, e:&mut ElseStmt<'a>) -> R;
    
}




#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum UnaryOperator {
    Plus,
    Minus,
    Increment,
    Decrement,
    Not,
    
}



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum PostfixOperator {
    Increment,
    Decrement,
    
}



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Is,
    
}



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum LogicalOperator {
    And,
    Or,
    
}



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum AssignmentOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    
}



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Expr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Lookup(LookupExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Literal(LiteralExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Call(CallExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Member(MemberExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Index(IndexExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Filter(FilterExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Group(GroupExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Binary(BinaryExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Logical(LogicalExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Postfix(PostfixExpr<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Unary(UnaryExpr<'a>),
    
}


impl<'a> Expr<'a> {
    pub fn accept<R>(&mut self, visitor: &mut dyn ExprVisitor<'a, R>) -> R {
        match self {
            Expr::Lookup(s) => visitor.visit_lookup_expr(s),
            Expr::Literal(s) => visitor.visit_literal_expr(s),
            Expr::Call(s) => visitor.visit_call_expr(s),
            Expr::Member(s) => visitor.visit_member_expr(s),
            Expr::Index(s) => visitor.visit_index_expr(s),
            Expr::Filter(s) => visitor.visit_filter_expr(s),
            Expr::Group(s) => visitor.visit_group_expr(s),
            Expr::Binary(s) => visitor.visit_binary_expr(s),
            Expr::Logical(s) => visitor.visit_logical_expr(s),
            Expr::Postfix(s) => visitor.visit_postfix_expr(s),
            Expr::Unary(s) => visitor.visit_unary_expr(s),
            
        }
    }
}



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Stmt<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Module(ModuleStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Template(TemplateStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Block(BlockStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Param(ParamStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Raw(RawStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Model(ModelStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Expr(ExprStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Loop(LoopStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Comment(CommentStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Tag(TagStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    If(IfStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Elif(ElifStmt<'a>),
    #[cfg_attr(feature = "serde", serde(borrow))]
    Else(ElseStmt<'a>),
    
}


impl<'a> Stmt<'a> {
    pub fn accept<R>(&mut self, visitor: &mut dyn StmtVisitor<'a, R>) -> R {
        match self {
            Stmt::Module(s) => visitor.visit_module_stmt(s),
            Stmt::Template(s) => visitor.visit_template_stmt(s),
            Stmt::Block(s) => visitor.visit_block_stmt(s),
            Stmt::Param(s) => visitor.visit_param_stmt(s),
            Stmt::Raw(s) => visitor.visit_raw_stmt(s),
            Stmt::Model(s) => visitor.visit_model_stmt(s),
            Stmt::Expr(s) => visitor.visit_expr_stmt(s),
            Stmt::Loop(s) => visitor.visit_loop_stmt(s),
            Stmt::Comment(s) => visitor.visit_comment_stmt(s),
            Stmt::Tag(s) => visitor.visit_tag_stmt(s),
            Stmt::If(s) => visitor.visit_if_stmt(s),
            Stmt::Elif(s) => visitor.visit_elif_stmt(s),
            Stmt::Else(s) => visitor.visit_else_stmt(s),
            
        }
    }
}





#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LookupExpr<'a> {
    pub value: Identifier<'a>,
    
}

impl<'a> LookupExpr<'a> {
    pub fn new(value: Identifier<'a>, ) -> LookupExpr<'a> {
        LookupExpr {
            value,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Literal<'a>,
    
}

impl<'a> LiteralExpr<'a> {
    pub fn new(value: Literal<'a>, ) -> LiteralExpr<'a> {
        LiteralExpr {
            value,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub member: Box<Expr<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub arguments: Vec<Expr<'a>>,
    
}

impl<'a> CallExpr<'a> {
    pub fn new(member: Box<Expr<'a>>, arguments: Vec<Expr<'a>>, ) -> CallExpr<'a> {
        CallExpr {
            member,
            arguments,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub object: Box<Expr<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub property: Box<Expr<'a>>,
    
}

impl<'a> MemberExpr<'a> {
    pub fn new(object: Box<Expr<'a>>, property: Box<Expr<'a>>, ) -> MemberExpr<'a> {
        MemberExpr {
            object,
            property,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub object: Box<Expr<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub index: Box<Expr<'a>>,
    
}

impl<'a> IndexExpr<'a> {
    pub fn new(object: Box<Expr<'a>>, index: Box<Expr<'a>>, ) -> IndexExpr<'a> {
        IndexExpr {
            object,
            index,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct FilterExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub object: Box<Expr<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub filter: Box<Expr<'a>>,
    
}

impl<'a> FilterExpr<'a> {
    pub fn new(object: Box<Expr<'a>>, filter: Box<Expr<'a>>, ) -> FilterExpr<'a> {
        FilterExpr {
            object,
            filter,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct GroupExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub expr: Box<Expr<'a>>,
    
}

impl<'a> GroupExpr<'a> {
    pub fn new(expr: Box<Expr<'a>>, ) -> GroupExpr<'a> {
        GroupExpr {
            expr,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub left: Box<Expr<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub right: Box<Expr<'a>>,
    pub op: BinaryOperator,
    
}

impl<'a> BinaryExpr<'a> {
    pub fn new(left: Box<Expr<'a>>, right: Box<Expr<'a>>, op: BinaryOperator, ) -> BinaryExpr<'a> {
        BinaryExpr {
            left,
            right,
            op,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub left: Box<Expr<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub right: Box<Expr<'a>>,
    pub op: LogicalOperator,
    
}

impl<'a> LogicalExpr<'a> {
    pub fn new(left: Box<Expr<'a>>, right: Box<Expr<'a>>, op: LogicalOperator, ) -> LogicalExpr<'a> {
        LogicalExpr {
            left,
            right,
            op,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct PostfixExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Box<Expr<'a>>,
    pub op: PostfixOperator,
    
}

impl<'a> PostfixExpr<'a> {
    pub fn new(value: Box<Expr<'a>>, op: PostfixOperator, ) -> PostfixExpr<'a> {
        PostfixExpr {
            value,
            op,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Box<Expr<'a>>,
    pub op: UnaryOperator,
    
}

impl<'a> UnaryExpr<'a> {
    pub fn new(value: Box<Expr<'a>>, op: UnaryOperator, ) -> UnaryExpr<'a> {
        UnaryExpr {
            value,
            op,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleStmt<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub statements: Vec<Stmt<'a>>,
    
}

impl<'a> ModuleStmt<'a> {
    pub fn new(statements: Vec<Stmt<'a>>, ) -> ModuleStmt<'a> {
        ModuleStmt {
            statements,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateStmt<'a> {
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub params: Vec<Stmt<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub body: Box<Stmt<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub extends: Option<Identifier<'a>>,
    
}

impl<'a> TemplateStmt<'a> {
    pub fn new(name: Cow<'a, str>, params: Vec<Stmt<'a>>, body: Box<Stmt<'a>>, extends: Option<Identifier<'a>>, ) -> TemplateStmt<'a> {
        TemplateStmt {
            name,
            params,
            body,
            extends,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Vec<Stmt<'a>>,
    
}

impl<'a> BlockStmt<'a> {
    pub fn new(value: Vec<Stmt<'a>>, ) -> BlockStmt<'a> {
        BlockStmt {
            value,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ParamStmt<'a> {
    pub name: Cow<'a, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ty: Type<'a>,
    
}

impl<'a> ParamStmt<'a> {
    pub fn new(name: Cow<'a, str>, ty: Type<'a>, ) -> ParamStmt<'a> {
        ParamStmt {
            name,
            ty,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct RawStmt<'a> {
    pub value: Cow<'a, str>,
    
}

impl<'a> RawStmt<'a> {
    pub fn new(value: Cow<'a, str>, ) -> RawStmt<'a> {
        RawStmt {
            value,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ModelStmt<'a> {
    pub name: Identifier<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub fields: Vec<Stmt<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub extends: Option<Identifier<'a>>,
    
}

impl<'a> ModelStmt<'a> {
    pub fn new(name: Identifier<'a>, fields: Vec<Stmt<'a>>, extends: Option<Identifier<'a>>, ) -> ModelStmt<'a> {
        ModelStmt {
            name,
            fields,
            extends,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ExprStmt<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Expr<'a>,
    
}

impl<'a> ExprStmt<'a> {
    pub fn new(value: Expr<'a>, ) -> ExprStmt<'a> {
        ExprStmt {
            value,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LoopStmt<'a> {
    pub key: Identifier<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub value: Option<Identifier<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub target: Expr<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub block: Box<Stmt<'a>>,
    
}

impl<'a> LoopStmt<'a> {
    pub fn new(key: Identifier<'a>, value: Option<Identifier<'a>>, target: Expr<'a>, block: Box<Stmt<'a>>, ) -> LoopStmt<'a> {
        LoopStmt {
            key,
            value,
            target,
            block,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CommentStmt<'a> {
    pub value: Cow<'a, str>,
    
}

impl<'a> CommentStmt<'a> {
    pub fn new(value: Cow<'a, str>, ) -> CommentStmt<'a> {
        CommentStmt {
            value,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TagStmt<'a> {
    pub name: Identifier<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub argument: Option<Expr<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub block: Option<Box<Stmt<'a>>>,
    
}

impl<'a> TagStmt<'a> {
    pub fn new(name: Identifier<'a>, argument: Option<Expr<'a>>, block: Option<Box<Stmt<'a>>>, ) -> TagStmt<'a> {
        TagStmt {
            name,
            argument,
            block,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub condition: Expr<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub consequence: Box<Stmt<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub alternative: Option<Box<Stmt<'a>>>,
    
}

impl<'a> IfStmt<'a> {
    pub fn new(condition: Expr<'a>, consequence: Box<Stmt<'a>>, alternative: Option<Box<Stmt<'a>>>, ) -> IfStmt<'a> {
        IfStmt {
            condition,
            consequence,
            alternative,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ElifStmt<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub condition: Expr<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub consequence: Box<Stmt<'a>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub alternative: Option<Box<Stmt<'a>>>,
    
}

impl<'a> ElifStmt<'a> {
    pub fn new(condition: Expr<'a>, consequence: Box<Stmt<'a>>, alternative: Option<Box<Stmt<'a>>>, ) -> ElifStmt<'a> {
        ElifStmt {
            condition,
            consequence,
            alternative,
            
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ElseStmt<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub block: Box<Stmt<'a>>,
    
}

impl<'a> ElseStmt<'a> {
    pub fn new(block: Box<Stmt<'a>>, ) -> ElseStmt<'a> {
        ElseStmt {
            block,
            
        }
    }
}

