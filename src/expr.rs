struct Expr {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Expr {
    pub fn new(left: Expr, operator: Token, right: Expr) {
        return Self {
            left,
            operator,
            right,
        };
    }
}
