use crate::error;
use crate::error::*;
use crate::expr::*;
use crate::object;
use crate::object::*;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, CDSyntaxError> {
        return Ok(expr.value.unwrap());
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, CDSyntaxError> {
        return self.evaluate(*expr.expression);
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, CDSyntaxError> {
        let right: Object = self.evaluate(*expr.right)?;

        match expr.operator.ttype {
            TokenType::BANG => {
                if self.is_truthy(&right) {
                    Ok(Object::False)
                } else {
                    Ok(Object::True)
                }
            }
            TokenType::MINUS => match right {
                Object::Num(num) => Ok(Object::Num(-num)),
                _ => Err(CDSyntaxError::error(
                    CDSyntaxErrorTypes::ENEXPECTED_TOKEN,
                    0,
                    0,
                    "Syntax Error".to_string(),
                    "Operand must be a number.".to_string(),
                )),
            },
            _ => Err(CDSyntaxError::error(
                CDSyntaxErrorTypes::ENEXPECTED_TOKEN,
                0,
                0,
                "Syntax Error".to_string(),
                "Invalid unary operator.".to_string(),
            )),
        }
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, CDSyntaxError> {
        return self.evaluate(expr.expression);
    }
}

impl Interpreter {
    fn evaluate(&self, expr: Expr) -> Result<Object, CDSyntaxError> {
        return expr.accept(self);
    }

    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Nil | Object::False)
    }
}
