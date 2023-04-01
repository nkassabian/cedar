enum Expr {
    Binary(BineryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub struct BineryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

pub struct LiteralExpr {
    value: Object,
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, KayLanError>;
    fn visit_grouping_expr(&self, expr: &BinaryExpr) -> Result<T, KayLanError>;
    fn visit_literal_expr(&self, expr: &BinaryExpr) -> Result<T, KayLanError>;
    fn visit_unary_expr(&self, expr: &BinaryExpr) -> Result<T, KayLanError>;
}

impl ExprVisitor {
    fn accept(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, KayLanError> {
        visitor.visit_binary_expr(self);
    }
}

impl GroupingExpr {
    fn accept(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, KayLanError> {
        visitor.visit_grouing_expr(self);
    }
}

impl LiteralExpr {
    fn accept(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, KayLanError> {
        visitor.visit_literal_expr(self);
    }
}

impl UnaryExpr {
    fn accept(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, KayLanError> {
        visitor.visit_unary_expr(self);
    }
}