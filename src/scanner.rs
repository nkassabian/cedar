use crate::error::KayLanError;
use crate::token::*;
use crate::token_type::*;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    position: usize,
    offset: usize,
    line: usize,
    file_name: String,
    current: usize,
}

impl Scanner {
    pub fn new(source: Vec<char>, file_name: String) -> Self {
        return Self {
            source,
            tokens: Vec::new(),
            position: 0,
            line: 0,
            offset: 0,
            current: 0,
            file_name,
        };
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, KayLanError> {
        while !self.is_eof() {
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report();
                    break;
                }
            }
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Object::Nil,
            self.line,
            self.offset,
        ));
        Ok(&self.tokens)
    }

    fn is_eof(&mut self) -> bool {
        return self.position >= self.source.len();
    }

    fn at(&mut self) -> char {
        if self.position != self.source.len() {
            return self.source[self.position];
        } else {
            return ' ';
        }
    }

    fn next_line(&mut self) {
        self.position += 1;
        self.offset = 0;
        self.line += 1;
    }

    fn add_token(&mut self, tok_type: TokenType) {
        let value = self.next();
        self.tokens.push(Token::new(
            tok_type,
            value.unwrap().to_string(),
            Object::Nil,
            self.line,
            self.offset,
        ))
    }

    fn peek(&mut self, char: char) -> bool {
        if self.is_eof() != true && self.position + 1 != self.source.len() {
            return self.source[self.position + 1] == char;
        } else {
            return false;
        }
    }

    fn peak_next(&mut self) -> char {
        if self.is_eof() != true && self.position + 1 != self.source.len() {
            return self.source[self.position + 1];
        } else {
            return '\0';
        }
    }

    fn add_conditional_token(
        &mut self,
        compare: char,
        type_true: TokenType,
        type_false: TokenType,
    ) {
        if self.peek(compare) {
            let value = format!(
                "{}{}",
                self.next().unwrap_or('\0').to_string(),
                self.next().unwrap_or('\0').to_string()
            );
            self.tokens.push(Token::new(
                type_true,
                value,
                Object::Nil,
                self.line,
                self.position,
            ))
        } else {
            match self.next() {
                Some(ch) => self.tokens.push(Token::new(
                    type_false,
                    ch.to_string(),
                    Object::Nil,
                    self.line,
                    self.offset,
                )),
                None => (),
            }
        }
    }

    fn check_for_comments(&mut self) {
        if self.peek('/') {
            self.next();
            self.next();
            while !self.peek('\n') && !self.is_eof() {
                self.next();
            }
            self.next();
        } else {
            self.add_token(TokenType::SLASH);
        }
    }

    fn empty_next(&mut self) {
        self.position += 1;
        self.offset += 1;
    }

    fn string(&mut self) -> Result<(), KayLanError> {
        println!("{}", "In string func");
        self.next();

        while !self.peek('"') && !self.is_eof() {
            if !self.peek('\n') {
                self.next_line();
            }
        }
        if self.is_eof() {
            return Err(KayLanError::error(
                self.line,
                self.offset,
                "Lexer Error".to_string(),
                format!("{} \"{}\".", "Unexpecter end of string. Expected", "\""),
                self.file_name.clone(),
            ));
        }

        self.next();
        self.next();

        let value = self.source[self.current + 1..self.position - 1]
            .iter()
            .collect();
        self.add_string_token(Object::Nil, TokenType::STRING, value);
        Ok(())
    }

    fn add_string_token(&mut self, object_type: Object, tok_type: TokenType, value: String) {
        self.tokens.push(Token::new(
            tok_type,
            value,
            object_type,
            self.line,
            self.current,
        ))
    }

    fn number(&mut self) -> Result<(), KayLanError> {
        while {
            let next = self.peak_next();
            self.is_digit(next)
        } {
            self.next();
        }

        //Look for floating point
        if self.peek('.') {
            self.next();
            let next = self.peak_next();
            if self.is_digit(next) {
                self.next();
                while {
                    let next = self.peak_next();
                    self.is_digit(next)
                } {
                    self.next();
                }
            } else {
                return Err(KayLanError::error(
                    self.line,
                    self.offset,
                    "Lexer Error".to_string(),
                    format!(
                        "{}: \"{}\".",
                        "Invalid end of number.", "Numbers cannot endw with a floating point."
                    ),
                    self.file_name.clone(),
                ));
            }
        }

        self.next();
        let value: String = self.source[self.current..self.position].iter().collect();
        let number = match value.parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                return Err(KayLanError::error(
                    self.line,
                    self.offset,
                    "Lexer Error".to_string(),
                    format!("{} \"{}\".", "Invalid number", value),
                    self.file_name.clone(),
                ));
            }
        };
        self.add_string_token(
            Object::Num(number),
            TokenType::NUMBER,
            self.source[self.current..self.position].iter().collect(),
        );
        Ok(())
    }

    fn is_digit(&mut self, char: char) -> bool {
        return char >= '0' && char <= '9';
    }

    fn is_alpha(&mut self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(&mut self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn identifier(&mut self) {
        while {
            let next = self.peak_next();

            self.is_alpha_numeric(next)
        } {
            self.next();
        }
        self.next();

        let text: String = self.source[self.current..self.position].iter().collect();
        let ttype = KEYWORDS
            .get(&text)
            .cloned()
            .unwrap_or(TokenType::IDENTIFIER);

        self.add_string_token(
            Object::Nil,
            ttype,
            self.source[self.current..self.position].iter().collect(),
        );
    }

    fn scan_token(&mut self) -> Result<(), KayLanError> {
        while !self.is_eof() {
            let c = self.at();
            self.current = self.position;

            match c {
                '\n' => self.next_line(),
                ' ' => self.empty_next(),
                '(' => self.add_token(TokenType::LEFTPAREN),
                ')' => self.add_token(TokenType::RIGHTPAREN),
                '{' => self.add_token(TokenType::LEFTBRACE),
                '}' => self.add_token(TokenType::RIGHTBRACE),
                '+' => self.add_token(TokenType::PLUS),
                '-' => self.add_token(TokenType::MINUS),
                '*' => self.add_token(TokenType::STAR),
                '.' => self.add_token(TokenType::DOT),
                ',' => self.add_token(TokenType::COMMA),
                ';' => self.add_token(TokenType::SEMICOLON),
                '!' => self.add_conditional_token('=', TokenType::BANGEQUAL, TokenType::BANG),
                '=' => self.add_conditional_token('=', TokenType::EQUALEQUAL, TokenType::EQUAL),
                '<' => self.add_conditional_token('=', TokenType::LESSEQUAL, TokenType::LESS),
                '>' => self.add_conditional_token('=', TokenType::GREATEREQUAL, TokenType::GREATER),
                '/' => self.check_for_comments(),
                '"' => self.string()?,
                _ => {
                    if self.is_digit(c) {
                        self.number()?;
                    } else if self.is_alpha(c) {
                        self.identifier();
                    } else {
                        return Err(KayLanError::error(
                            self.line,
                            self.offset,
                            "Lexer Error".to_string(),
                            format!("{} \"{}\".", "Unexpecter charater :", c.to_string()),
                            self.file_name.clone(),
                        ));
                    }
                } // {

                  // }
            }
        }
        Ok(())
    }
}

impl Iterator for Scanner {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if !self.is_eof() {
            self.offset += 1;
            let char = self.source[self.position];
            self.position += 1;
            return Some(char);
        } else {
            return None;
        }
    }
}
