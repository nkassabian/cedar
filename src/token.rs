use once_cell::sync::Lazy;

use crate::token_type::*;
use core::fmt;
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "Nil"),
            Object::True => write!(f, "True"),
            Object::False => write!(f, "False"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
    position: usize,
}

impl Token {
    pub fn new(
        ttype: TokenType,
        lexeme: String,
        literal: Object,
        line: usize,
        position: usize,
    ) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line,
            position,
        }
    }

    pub fn as_string(&self) -> String {
        self.lexeme.clone()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{:?} {} {} {} {} \n\n",
            self.ttype,
            self.lexeme,
            match &self.literal {
                Object::Num(x) => x.to_string(),
                Object::Str(x) => format!("\"{}\"", x),
                Object::Nil => "Nil".to_string(),
                Object::True => "True".to_string(),
                Object::False => "False".to_string(),
            },
            self.line,
            self.position
        )
    }
}

pub static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(String::from("var"), TokenType::VAR);
    map.insert(String::from("for"), TokenType::FOR);
    map.insert(String::from("have"), TokenType::HAVE);
    map.insert(String::from("func"), TokenType::FUNC);
    map.insert(String::from("else"), TokenType::ELSE);
    map.insert(String::from("class"), TokenType::CLASS);
    map.insert(String::from("if"), TokenType::IF);
    map.insert(String::from("&&"), TokenType::AND);
    map.insert(String::from("||"), TokenType::OR);
    map.insert(String::from("while"), TokenType::WHILE);
    map.insert(String::from("show"), TokenType::PRINT);
    map.insert(String::from("ret"), TokenType::RETURN);
    map.insert(String::from("null"), TokenType::NIL);
    map.insert(String::from("true"), TokenType::TRUE);
    map.insert(String::from("false"), TokenType::FALSE);

    map
});
