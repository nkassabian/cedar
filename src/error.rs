use ansi_term::Colour::{Black, Blue, Purple, Red, Yellow};

use crate::token::Token;

#[derive(Debug)]
pub struct CDLexerError {
    line: usize,
    message: String,
    column: usize,
    main_message: String,
    file_name: String,
    source_toks: Vec<char>,
}

impl CDLexerError {
    pub fn error(
        line: usize,
        column: usize,
        main_message: String,
        message: String,
        file_name: String,
        source_toks: Vec<char>,
    ) -> CDLexerError {
        CDLexerError {
            line,
            message,
            column,
            main_message,
            file_name,
            source_toks: source_toks,
        }
    }

    // [src/test.ql->1:20::Unexpected token found. of type: ;
    // 0 |
    // 1 | have shipSpeedX := 0;
    //                       ^^^ // this is the position of the token in the line above
    // 2 |
    pub fn report(self) {
        let mut line: String = "".to_string();

        for token in self.source_toks {
            line = format!("{}{}", line, token.to_string());
        }

        let mut error_line: String = " ".to_string();
        if self.line < 10 {
            error_line = format!(
                "    {}\n    {}\n    {}\n    {}\n",
                format!("{}", get_line(self.line + 1, &line, -1)),
                format!("{}", get_line(self.line + 1, &line, 0)),
                format!(
                    "{}",
                    Red.bold()
                        .paint(create_string_with_spaces(self.column + 2, '^'))
                ),
                format!("{}", get_line(self.line + 1, &line, 1))
            )
        }

        eprint!(
            "\n\n[{}]->{}::{}\n\n{}",
            Yellow.bold().paint(self.file_name),
            format!(
                "{}:{}",
                Red.bold().paint(self.line.to_string()),
                Purple.bold().paint(self.column.to_string())
            ),
            format!("{}, {}", Blue.bold().paint(self.main_message), self.message),
            error_line
        );
        std::process::exit(64);
    }
}

#[derive(Clone, Debug)]
pub enum CDSyntaxErrorTypes {
    UNEXPECTED_EOF,
    ENEXPECTED_TOKEN,
}

#[derive(Clone, Debug)]
pub struct CDSyntaxError {
    line: usize,
    column: usize,
    main_message: String,
    message: String,
    ErrorType: CDSyntaxErrorTypes,
}

impl CDSyntaxError {
    pub fn error(
        ErrorType: CDSyntaxErrorTypes,
        line: usize,
        column: usize,
        main_message: String,
        message: String,
    ) -> CDSyntaxError {
        CDSyntaxError {
            ErrorType,
            line,
            message,
            column,
            main_message,
        }
    }

    pub fn report(self) {
        eprintln!("{}: {}!", self.main_message, self.message);
        std::process::exit(64);
    }

    // TODO: Make dynamic
    pub fn runtime_error() {
        let err = CDSyntaxError::error(
            CDSyntaxErrorTypes::ENEXPECTED_TOKEN,
            0,
            0,
            "Syntax Error".to_string(),
            "Operand must be a number.".to_string(),
        );
        err.report();
    }
}

fn get_line(line: usize, error_line: &String, line_num: i32) -> String {
    if line_num == -1 {
        return format!(
            "{}\n",
            Black.paint(format!(
                "{} | {}",
                ("0".to_string() + &(line - 1).to_string()),
                &error_line[find_nth_index(&error_line, '\n', line - 2).unwrap() + 1
                    ..find_nth_index(&error_line, '\n', line - 1).unwrap()]
            ))
        );
    } else if line_num == 0 {
        return format!(
            "{} | {}",
            ("0".to_string() + &line.to_string()),
            &error_line[find_nth_index(&error_line, '\n', line - 1).unwrap() + 1
                ..find_nth_index(&error_line, '\n', line).unwrap()]
        );
    } else if line_num == 1 {
        if (find_nth_index(&error_line, '\n', line).unwrap() + 1)
            != find_nth_index(&error_line, '\n', line + 1).unwrap() + 1
        {
            return format!(
                "{}\n",
                Black.paint(format!(
                    "{} | {}",
                    ("0".to_string() + &(line + 1).to_string()),
                    &error_line[find_nth_index(&error_line, '\n', line).unwrap() + 1
                        ..find_nth_index(&error_line, '\n', line + 1).unwrap()]
                ))
            );
        } else {
            return " ".to_string();
        }
    } else {
        return " ".to_string();
    }
}

fn create_string_with_spaces(x: usize, c: char) -> String {
    let mut s = String::new();
    for _ in 0..x + 3 {
        s.push(' ');
    }
    s.push(c);
    s
}

fn find_nth_index(s: &str, c: char, n: usize) -> Option<usize> {
    let mut iter = s.char_indices().filter(|&(_, ch)| ch == c).skip(n - 1);
    match iter.next() {
        Some((i, _)) => Some(i),
        None => Some(s.len()),
    }
}
