use crate::error::*;
use crate::expr::*;
use crate::tokens::token::*;
use crate::errors::syntax_error::*;

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, SyntaxError> {
        match self {
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
            Stmt::Var(v) => v.accept(stmt_visitor),
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, SyntaxError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, SyntaxError>;
    fn visit_var_stmt(&self, expr: &VarStmt) -> Result<T, SyntaxError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, SyntaxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, SyntaxError> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, SyntaxError> {
        visitor.visit_var_stmt(self)
    }
}

