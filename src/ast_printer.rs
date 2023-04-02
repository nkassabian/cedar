use crate::error::*;
use crate::expr::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, CDLexerError> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &String, exprs: &[&Box<Expr>]) -> Result<String, CDLexerError> {
        let mut builder = format!("({name}");

        for expr in exprs {
            builder = format!("{builder} {}", expr.accept(self)?);
        }
        builder = format!("{builder})");

        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, CDLexerError> {
        self.parenthesize(&expr.operator.as_string(), &[&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, CDLexerError> {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, CDLexerError> {
        if let Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, CDLexerError> {
        self.parenthesize(&expr.operator.as_string(), &[&expr.right])
    }
}
