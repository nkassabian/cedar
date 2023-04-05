use crate::error::*;
use crate::expr::*;
use crate::object::*;
use crate::stmt::ExpressionStmt;
use crate::stmt::PrintStmt;
use crate::stmt::Stmt;
use crate::stmt::VarStmt;
use crate::token::*;
use crate::token_type::*;
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, CDSyntaxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.decleration()?);
        }
        return Ok(statements);
    }
    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::SEMICOLON {
                match self.peek().ttype {
                    TokenType::CLASS => return,
                    TokenType::LEFTPAREN => return,
                    TokenType::RIGHTPAREN => return,
                    TokenType::LEFTBRACE => return,
                    TokenType::RIGHTBRACE => return,
                    TokenType::COMMA => return,
                    TokenType::DOT => return,
                    TokenType::MINUS => return,
                    TokenType::PLUS => return,
                    TokenType::SEMICOLON => return,
                    TokenType::SLASH => return,
                    TokenType::STAR => return,
                    TokenType::BANG => return,
                    TokenType::BANGEQUAL => return,
                    TokenType::EQUAL => return,
                    TokenType::EQUALEQUAL => return,
                    TokenType::GREATER => return,
                    TokenType::GREATEREQUAL => return,
                    TokenType::LESS => return,
                    TokenType::LESSEQUAL => return,
                    TokenType::IDENTIFIER => return,
                    TokenType::STRING => return,
                    TokenType::NUMBER => return,
                    TokenType::AND => return,
                    TokenType::ELSE => return,
                    TokenType::FALSE => return,
                    TokenType::FUNC => return,
                    TokenType::FOR => return,
                    TokenType::IF => return,
                    TokenType::NIL => return,
                    TokenType::OR => return,
                    TokenType::PRINT => return,
                    TokenType::RETURN => return,
                    TokenType::TRUE => return,
                    TokenType::HAVE => return,
                    TokenType::VAR => return,
                    TokenType::WHILE => return,
                    TokenType::EOF => return,
                }
            }
            self.advance();
        }
    }

    pub fn expression(&mut self) -> Result<Expr, CDSyntaxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, CDSyntaxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BANGEQUAL, TokenType::EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, CDSyntaxError> {
        let mut expr = self.term()?;

        while self.is_match(&[
            TokenType::GREATEREQUAL,
            TokenType::GREATER,
            TokenType::LESSEQUAL,
            TokenType::LESS,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, CDSyntaxError> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, CDSyntaxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Ok(Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }))?;
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, CDSyntaxError> {
        if self.is_match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, CDSyntaxError> {
        if self.is_match(&[TokenType::TRUE]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            }));
        }

        if self.is_match(&[TokenType::FALSE]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            }));
        }

        if self.is_match(&[TokenType::NIL]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        if self.is_match(&[TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(self.previous().literal),
            }));
        }

        if self.is_match(&[TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous(),
            }));
        }
        if self.is_match(&[TokenType::LEFTPAREN]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RIGHTPAREN,
                "Expect ')' after expression".to_string(),
            )?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        if self.is_match(&[TokenType::PRINT]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous().clone(),
            }));
        }

        if self.is_match(&[TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous(),
            }));
        }

        Err(CDSyntaxError::error(
            CDSyntaxErrorTypes::ENEXPECTED_TOKEN,
            0,
            0,
            "Syntax Error".to_string(),
            "unrecognized error".to_string(),
        ))
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, CDSyntaxError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            let p = self.peek();
            Err(CDSyntaxError::error(
                CDSyntaxErrorTypes::ENEXPECTED_TOKEN,
                p.line,
                0,
                "Syntax Error".to_string(),
                message.to_string(),
            ))
        }
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ttype == ttype
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    /*
    ==============================
            STATEMENTS
    ==============================
     */

    fn statement(&mut self) -> Result<Stmt, CDSyntaxError> {
        if self.is_match(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn decleration(&mut self) -> Result<Stmt, CDSyntaxError> {
        if self.is_match(&[TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        }
        .or_else(|_| {
            self.synchronize();
            Err(CDSyntaxError::error(
                CDSyntaxErrorTypes::ENEXPECTED_TOKEN,
                0,
                0,
                "Syntax Error".to_string(),
                "Error in decleration".to_string(),
            ))
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, CDSyntaxError> {
        let name: Token = self
            .consume(TokenType::IDENTIFIER, "Expected variable name".to_string())
            .unwrap();

        let initializer = match self.is_match(&[TokenType::EQUAL]) {
            true => Some(self.expression()?),
            false => None,
        };

        self.consume(
            TokenType::SEMICOLON,
            "Expected ';' after variable decleration".to_string(),
        )?;

        return Ok(Stmt::Var(VarStmt {
            name: name,
            initializer,
        }));
    }

    fn print_statement(&mut self) -> Result<Stmt, CDSyntaxError> {
        let value = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, CDSyntaxError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }
}
