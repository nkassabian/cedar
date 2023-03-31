use once_cell::sync::Lazy;

use crate::token_type::*;
use core::fmt;
use std::collections::HashMap;
#[derive(Debug)]
pub enum Object {
    Num(f64),
    //Str(String),
    Nil,
    //True,
    //False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            //Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "Nil"),
            //Object::True => write!(f, "True"),
            //Object::False => write!(f, "False"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Object,
    line: usize,
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
                //Object::Str(x) => format!("\"{}\"", x),
                Object::Nil => "Nil".to_string(),
                //Object::True => "True".to_string(),
                //Object::False => "False".to_string(),
            },
            self.line,
            self.position
        )
    }
}

pub static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(String::from("var"), TokenType::VAR);

    map
});
