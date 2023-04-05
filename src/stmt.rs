use crate::error::*;
use crate::expr::*;
use crate::token::*;

#[derive(Debug)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, CDSyntaxError> {
        match self {
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
            Stmt::Var(v) => v.accept(stmt_visitor),
        }
    }
}
#[derive(Debug)]

pub struct ExpressionStmt {
    pub expression: Expr,
}
#[derive(Debug)]

pub struct PrintStmt {
    pub expression: Expr,
}
#[derive(Debug)]

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, CDSyntaxError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, CDSyntaxError>;
    fn visit_var_stmt(&self, expr: &VarStmt) -> Result<T, CDSyntaxError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, CDSyntaxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, CDSyntaxError> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, CDSyntaxError> {
        visitor.visit_var_stmt(self)
    }
}
