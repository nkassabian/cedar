use crate::errors::lexer_error::LexerError;
use crate::errors::lexer_error::LexerErrorTypes;
use crate::object::*;
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

    /// Scans the source code and generates a vector of tokens. It
    /// keeps calling the scan_token function until the end of the
    /// source code is reached. If an error is encountered while
    /// scanning a token, it is reported and the function stops scanning.
    /// At the end, an EOF token is added to the token vector, and the
    /// function returns a reference to the vector of tokens, wrapped
    /// in a Result indicating success or an error of type LexerError.
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LexerError> {
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

    /// Returns a boolean indicating whether the parser has reached
    /// the end of the source code being parsed. If the current
    /// position of the parser is greater than or equal to the length
    ///  of the source code, it is considered to have reached the end
    /// and returns true, otherwise it returns false.
    fn is_eof(&mut self) -> bool {
        return self.position >= self.source.len();
    }

    /// Returns the current character at the current position in the
    /// source code string. If the current position is at the end of
    /// the source string, a space character is returned instead.
    fn at(&mut self) -> char {
        if self.position != self.source.len() {
            return self.source[self.position];
        } else {
            return ' ';
        }
    }

    /// Increments the current position 1, resets the character offset,
    /// and increments the line number by 1. This effectively moves
    /// the scanner to the start of the next line in the source code.
    fn next_line(&mut self) {
        self.position += 1;
        self.offset = 0;
        self.line += 1;
    }

    /// Adds a new token with the given tok_type to the list of tokens.
    /// The value of the token is the next character in the source code
    /// after the current position. The new token is created with the
    /// Token::new() method and pushed to the end of the tokens vector.
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

    /// Returns true if the next character in the source matches the
    /// specified character, false otherwise.If the end of the source
    /// is reached, returns false.
    fn peek(&mut self, c: char) -> bool {
        !self.is_eof()
            && self.position + 1 < self.source.len()
            && self.source[self.position + 1] == c
    }

    /// Returns the character immediately following the current position
    /// in the source string.If the end of the file is reached or if the
    /// next character is out of range, it returns the null character '\0'
    fn peak_next(&mut self) -> char {
        if !self.is_eof() && self.position + 1 != self.source.len() {
            self.source[self.position + 1]
        } else {
            '\0'
        }
    }

    /// This function adds a conditional token to the token list based
    /// on whether the next character is equal to the given comparison
    /// character. If it is, a token of type_true is added, otherwise
    /// a token of type_false is added. If there are no more characters
    /// in the input, nothing is added to the token list.
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
                self.offset,
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

    /// This function checks for comments in the input and updates
    /// the tokenizer state accordingly, handling single-line and  
    /// comments and multi-line comments.
    fn check_for_comments(&mut self) {
        //Check for multi line
        //TODO: Add multiline comments
        if self.peek('-') {
            self.next();
            self.next();
            while !self.peek('\n') && !self.is_eof() {
                self.next();
            }
            self.next_line();
        }

        if self.peek('/') {
            self.next();
            self.next();
            while !self.peek('\n') && !self.is_eof() {
                self.next();
            }
            self.next_line();
        } else {
            self.add_token(TokenType::SLASH);
        }
    }

    /// Increments the current position and offset of the scanner by 1,
    /// effectively advancing to the next character without doing
    /// anything else. It is typically used to skip over whitespace or
    /// other non-significant characters.
    fn empty_next(&mut self) {
        self.position += 1;
        self.offset += 1;
    }

    /// This function extracts a string token from the input and updates
    /// the tokenizer state accordingly. It scans the input for the end
    /// of the string while handling newline characters appropriately.
    /// If the end of the input is reached before the end of the string
    /// is found, it returns a Lexer Error. If the string is successfully
    /// parsed, a string token is added to the tokenizer state.
    // TODO: Add check for new line
    fn string(&mut self) -> Result<(), LexerError> {
        self.next();
        while !self.peek('"') && !self.is_eof() {
            self.next();
            if self.peek('\n') {
                self.next_line();
            }
        }
        if self.is_eof() {
            return Err(LexerError::new(
                self.line,
                self.offset,
                LexerErrorTypes::UnexpectedEndOfString,
                self.file_name.clone(),
                self.source.clone(),
            ));
        }

        self.next();
        self.next();

        let value: String = self.source[self.current + 1..self.position - 1]
            .iter()
            .collect();
        self.add_string_token(Object::Str(value.clone()), TokenType::STRING, value);
        Ok(())
    }

    /// This function adds a new string token to the tokenizer state with
    /// the provided object type, token type, and string value. It creates
    /// a new Token instance and pushes it onto the tokens vector in the
    /// tokenizer state, updating the line and current position information
    /// as appropriate.
    fn add_string_token(&mut self, object_type: Object, tok_type: TokenType, value: String) {
        self.tokens.push(Token::new(
            tok_type,
            value,
            object_type,
            self.line,
            self.offset,
        ))
    }

    /// Extracts a number token from the input and updates the
    /// tokenizer state accordingly. It scans the input for the end of the
    /// number, handling floating-point values if present. If the number
    /// is successfully parsed, a number token is added to the tokenizer
    /// state with the appropriate object type (Num) and token type (NUMBER).
    /// If an invalid number is encountered, it returns a Lexer Error.
    fn number(&mut self) -> Result<(), LexerError> {
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
                return Err(LexerError::new(
                    self.line,
                    self.offset,
                    LexerErrorTypes::InvalidFloatingPoint(),
                    self.file_name.clone(),
                    self.source.clone(),
                ));
            }
        }

        self.next();
        let value: String = self.source[self.current..self.position].iter().collect();
        let number = match value.parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                return Err(LexerError::new(
                    self.line,
                    self.offset,
                    LexerErrorTypes::InvalidNumber(value),
                    self.file_name.clone(),
                    self.source.clone(),
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

    /// This function checks if a character is a digit (0-9) and returns a
    /// boolean value accordingly.
    fn is_digit(&mut self, char: char) -> bool {
        return char >= '0' && char <= '9';
    }

    /// This function checks if a character is an alphabetic character,
    /// underscore (_), vertical bar (|), or ampersand (&), and returns
    /// a boolean value accordingly.
    fn is_alpha(&mut self, c: char) -> bool {
        return (c >= 'a' && c <= 'z')
            || (c >= 'A' && c <= 'Z')
            || c == '_'
            || c == '|'
            || c == '&';
    }

    /// This function checks if a character is either an alphabetic
    /// character or a digit (0-9), using the is_alpha and is_digit
    /// helper functions, and returns a boolean value accordingly.
    fn is_alpha_numeric(&mut self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    /// This function scans an identifier by repeatedly advancing
    /// the position until the next character is not an alphanumeric
    /// character. It then creates a new string from the slice of the
    /// source code between the current and next positions, and looks
    /// up the corresponding TokenType using the KEYWORDS map. Finally,
    /// it adds a new Token to the tokens vector using the add_string_token
    /// helper function, with the appropriate TokenType and value fields.
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

        // TODO: CLEAN bool value
        let mut bool_value = Some(false);
        if text == "false" {
            bool_value = Some(false);
        } else {
            bool_value = Some(true);
        }

        let object = match bool_value {
            Some(bool_value) => Object::Bool(bool_value),
            None => Object::Str(text.to_string()),
        };

        self.add_string_token(
            object,
            ttype,
            self.source[self.current..self.position].iter().collect(),
        );
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
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
                        return Err(LexerError::new(
                            self.line,
                            self.offset,
                            LexerErrorTypes::UnexpectedCharacter(c),
                            self.file_name.clone(),
                            self.source.clone(),
                        ));
                        // return Err(LexerError::error(
                        //     self.line,
                        //     self.offset,
                        //     "Lexer Error".to_string(),
                        //     format!("{} \"{}\".", "Unexpecter charater :", c.to_string()),
                        //     self.file_name.clone(),
                        //     self.source.clone(),
                        // ));
                    }
                }
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
