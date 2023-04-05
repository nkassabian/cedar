use std::os::raw::c_void;

use crate::error::*;
use crate::expr::*;

use crate::object::*;

use crate::stmt;
use crate::stmt::ExpressionStmt;
use crate::stmt::PrintStmt;
use crate::stmt::Stmt;
use crate::stmt::StmtVisitor;
use crate::stmt::VarStmt;
use crate::token::Token;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, CDSyntaxError> {
        return Ok(expr.value.clone().unwrap());
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, CDSyntaxError> {
        return self.evaluate(&expr.expression);
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, CDSyntaxError> {
        let right: Object = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::BANG => {
                if self.is_truthy(&right) {
                    Ok(Object::Bool(false))
                } else {
                    Ok(Object::Bool(true))
                }
            }
            TokenType::MINUS => {
                Interpreter::checkNumberOperand(expr.operator.clone(), &right)?;

                match right {
                    Object::Num(num) => Ok(Object::Num(-num)),
                    _ => Err(CDSyntaxError::error(
                        CDSyntaxErrorTypes::ENEXPECTED_TOKEN,
                        0,
                        0,
                        "Syntax Error".to_string(),
                        "Operand must be a number.".to_string(),
                    )),
                }
            }
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
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        // TODO: Add String support for both expressions
        //Interpreter::check_number_operands(&expr.operator, &left, &right);
        let result = match expr.operator.ttype {
            TokenType::MINUS => left - right,
            TokenType::SLASH => left / right,
            TokenType::STAR => left * right,
            TokenType::PLUS => left + right,
            TokenType::GREATER => Object::Bool(left > right),
            TokenType::LESS => Object::Bool(left < right),
            TokenType::GREATEREQUAL => Object::Bool(left >= right),
            TokenType::LESSEQUAL => Object::Bool(left <= right),
            TokenType::EQUALEQUAL => Object::Bool(left == right),
            TokenType::BANGEQUAL => Object::Bool(left != right),

            _ => Object::ArithmeticError,
        };
        if result == Object::ArithmeticError {
            Err(CDSyntaxError::error(
                CDSyntaxErrorTypes::ENEXPECTED_TOKEN,
                0,
                0,
                "Syntax Error".to_string(),
                "Invalid unary operator.".to_string(),
            ))
        } else {
            Ok(result)
        }
    }

    fn visit_variable_expr(&self, _expr: &VariableExpr) -> Result<Object, CDSyntaxError> {
        Ok(Object::Nil)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<(), CDSyntaxError> {
        self.evaluate(&expr.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<(), CDSyntaxError> {
        self.evaluate(&expr.expression)?;
        Ok(())
    }

    fn visit_var_stmt(&self, _expr: &VarStmt) -> Result<(), CDSyntaxError> {
        Ok(())
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, CDSyntaxError> {
        return expr.accept(self);
    }

    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Nil | Object::Bool(false))
    }
    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(&statement);
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> () {
        stmt.accept(self).unwrap();
    }
    pub fn checkNumberOperand(operator: Token, operand: &Object) -> Result<(), CDSyntaxError> {
        if let Object::Num(_) = operand {
            Ok(())
        } else {
            println!("{}", "Error");
            Ok(CDSyntaxError::runtime_error())
        }
    }

    fn check_number_operands(
        operator: &Token,
        left: &Object,
        right: &Object,
    ) -> Result<(), CDSyntaxError> {
        if let (Object::Num(_), Object::Num(_)) = (left, right) {
            Ok(())
        } else {
            Ok(CDSyntaxError::runtime_error())
        }
    }
}
