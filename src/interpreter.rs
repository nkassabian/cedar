use crate::errors::syntax_error::SyntaxError;
use crate::errors::syntax_error::SyntaxErrorTypes;
use crate::expr::*;

use crate::object::*;

use crate::stmt::ExpressionStmt;
use crate::stmt::PrintStmt;
use crate::stmt::Stmt;
use crate::stmt::StmtVisitor;
use crate::stmt::VarStmt;
use crate::tokens::token::*;
use crate::tokens::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, SyntaxError> {
        return Ok(expr.value.clone().unwrap());
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, SyntaxError> {
        return self.evaluate(&expr.expression);
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, SyntaxError> {
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
                Interpreter::check_number_operand(expr.operator.clone(), &right)?;

                match right {
                    Object::Num(num) => Ok(Object::Num(-num)),
                    _ => Err(SyntaxError::new(0, 0, SyntaxErrorTypes::OperandNaN())),
                }
            }
            _ => Err(SyntaxError::new(0, 0, SyntaxErrorTypes::InvalidUnary())),
        }
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, SyntaxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
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
            Err(SyntaxError::new(0, 0, SyntaxErrorTypes::InvalidUnary()))
        } else {
            Ok(result)
        }
    }

    fn visit_variable_expr(&self, _expr: &VariableExpr) -> Result<Object, SyntaxError> {
        Ok(Object::Nil)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<(), SyntaxError> {
        self.evaluate(&expr.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<(), SyntaxError> {
        println!("{}", self.evaluate(&expr.expression)?.to_string());
        Ok(())
    }

    fn visit_var_stmt(&self, _expr: &VarStmt) -> Result<(), SyntaxError> {
        Ok(())
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, SyntaxError> {
        Ok(expr.accept(self)?)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Nil | Object::Bool(false))
    }
    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.execute(&statement);
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), SyntaxError> {
        Ok(stmt.accept(self)?)
    }
    pub fn check_number_operand(operator: Token, operand: &Object) -> Result<(), SyntaxError> {
        if let Object::Num(_) = operand {
            Ok(())
        } else {
            Err(SyntaxError::new(0, 0, SyntaxErrorTypes::InvalidUnary()))
        }
    }

    fn check_number_operands(
        operator: &Token,
        left: &Object,
        right: &Object,
    ) -> Result<(), SyntaxError> {
        if let (Object::Num(_), Object::Num(_)) = (left, right) {
            Ok(())
        } else {
            Err(SyntaxError::new(0, 0, SyntaxErrorTypes::InvalidUnary()))
        }
    }
}
