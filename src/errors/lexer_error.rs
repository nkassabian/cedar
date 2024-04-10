use ansi_term::Colour::{Blue, Purple, Red, Yellow};

#[derive(Debug, Clone)]
pub struct LexerError {
    error_type: LexerErrorTypes,
    line: usize,
    column: usize,
    main_message: String,
    file_name: String,
    source_toks: Vec<char>,
}

#[derive(Debug, Clone)]
pub enum LexerErrorTypes {
    UnexpectedEndOfString,
    UnexpectedCharacter(char),
    InvalidFloatingPoint(),
    InvalidNumber(String),
}

impl LexerError {
    pub fn new(
        line: usize,
        column: usize,
        error_type: LexerErrorTypes,
        file_name: String,
        source_toks: Vec<char>,
    ) -> LexerError {
        let main_message = match &error_type {
            LexerErrorTypes::UnexpectedEndOfString => {
                "error[E2502] Unexpected end of string".to_string()
            }
            LexerErrorTypes::UnexpectedCharacter(c) => {
                format!("error[E2503] Unexpected character '{}'", c)
            }
            LexerErrorTypes::InvalidFloatingPoint() => {
                format!("error[E2504] Invalid floating point.")
            }
            LexerErrorTypes::InvalidNumber(s) => format!("Invalid number '{}'", s),
        };
        LexerError {
            error_type,
            line,
            column,
            main_message,
            file_name,
            source_toks,
        }
    }
    fn message(&self) -> String {
        return match &self.error_type {
            LexerErrorTypes::UnexpectedEndOfString => "Unexpected end of string".to_string(),
            LexerErrorTypes::UnexpectedCharacter(c) => {
                format!("Tip: Try to remove or replace this character: {}.", c)
            }
            LexerErrorTypes::InvalidFloatingPoint() => {
                format!("Floating point should have a digit after it. \n ")
            }
            LexerErrorTypes::InvalidNumber(s) => format!("Invalid number '{}'", s),
        };
    }
    // [src/test.ql->1:20::Unexpected token found. of type: ;
    // 0 |
    // 1 | have shipSpeedX := 0;
    //                       ^^^ // this is the position of the token in the line above
    // 2 |

    ///reports an error by displaying the relevant code line,
    /// error message, and error location. It also exits the
    /// program with a status code of 64.
    pub fn report(self) {
        let mut line: String = "".to_string();

        for token in self.source_toks.clone() {
            line = format!("{}{}", line, token.to_string());
        }

        // TheDevConnor: 2023-04-08 12:00:00
        // I was able toi find a small bug. the error line was returning the wrong line number.
        // I fixed it by adding 1 to the line number.
        // The reason for this is because the line number is 0 indexed and the error line function is 1 indexed.
        // Hence we need to do the self.line + 1 to get the correct line number.
        let error_line: String = get_error_line(&self.source_toks, self.line).to_string();
        let mut error_message: String = "".to_string();
        if self.line < 10 {
            error_message = format!(
                "{}\n    {}\n",
                format!("{}", error_line),
                format!(
                    "{} {}",
                    Red.bold().paint(flash_error_location(self.column + 2,)),
                    Red.bold().paint(self.message())
                ),
            )
        }

        eprint!(
            "\n\n[{}] | [{}]:=> {}\n\n{}",
            Yellow.bold().paint(self.file_name),
            format!(
                "{}:{}",
                Red.bold().paint((self.line + 1).to_string()),
                Purple.bold().paint(self.column.to_string())
            ),
            format!(
                "{}\n{}",
                Red.bold().paint("Exception Occured"),
                Blue.bold().paint(self.main_message),
            ),
            error_message
        );
        std::process::exit(64);
    }
}

/// takes a vector of characters source_toks and a line number line_num,
/// and returns a formatted string representing the line of the given
/// line number. It first finds the start and end indices of the line
/// in the source_toks vector, then creates a substring of the line by
/// collecting the characters between those indices into a String. Finally,
/// it formats and returns the line as a string with the line number and a
/// leading zero followed by a vertical bar (|) separator.
fn get_error_line(source_toks: &[char], line_num: usize) -> String {
    let line_start = source_toks
        .iter()
        .enumerate()
        .find(|(_, &c)| c == '\n')
        .map(|(i, _)| i + 1)
        .unwrap_or(0);
    let line_end = source_toks
        .iter()
        .skip(line_start)
        .position(|&c| c == '\n')
        .map(|i| i + line_start)
        .unwrap_or(source_toks.len());
    let line = source_toks[line_start..line_end].iter().collect::<String>();
    format!("0{} | {}", line_num + 1, line)
}

fn flash_error_location(col_num: usize) -> String {
    let pointer = std::iter::repeat(' ')
        .take(col_num - 1)
        .chain(std::iter::once('^'))
        .collect::<String>();
    format!("{}", pointer)
}
