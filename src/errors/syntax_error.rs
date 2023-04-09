use ansi_term::Colour::Black;

#[derive(Debug)]
pub struct SyntaxError {
    error_type: SyntaxErrorTypes,
    line: usize,
    column: usize,
    main_message: String,
}

#[derive(Debug)]
pub enum SyntaxErrorTypes {
    UnexpectedToken(String),
    ExpectedToken(String, String),
    UnexpectedCharacter(char),
    UnclosedDelimiter(char),
    MissingOperand(String),
    RedundantOperand(String),
    InvalidSyntax(String),
    OperandNaN(),
    InvalidUnary(),
}

impl SyntaxError {
    pub fn new(line: usize, column: usize, error_type: SyntaxErrorTypes) -> SyntaxError {
        let main_message = match &error_type {
            SyntaxErrorTypes::UnexpectedToken(c) => {
                format!("{}: Token \"{}\"", "Unexpected Token".to_string(), c)
            }
            SyntaxErrorTypes::ExpectedToken(c, d) => {
                format!("Expected Token \"{}\", got \"{}\".", c, d)
            }
            SyntaxErrorTypes::OperandNaN() => {
                format!("Operand must be a number.")
            }
            SyntaxErrorTypes::InvalidUnary() => {
                format!("Invalid unary expression.")
            }
            SyntaxErrorTypes::UnexpectedCharacter(_) => todo!(),
            SyntaxErrorTypes::UnclosedDelimiter(_) => todo!(),
            SyntaxErrorTypes::MissingOperand(_) => todo!(),
            SyntaxErrorTypes::RedundantOperand(_) => todo!(),
            SyntaxErrorTypes::InvalidSyntax(_) => todo!(),
        };
        SyntaxError {
            error_type,
            line,
            column,
            main_message,
        }
    }

    // [src/test.ql->1:20::Unexpected token found. of type: ;
    // 0 |
    // 1 | have shipSpeedX := 0;
    //                       ^^^ // this is the position of the token in the line above
    // 2 |
    pub fn report(self) {
        eprint!(
            "[{}:{}] Syntax Error :=> {}",
            self.line + 1,
            self.column,
            self.main_message
        );
        std::process::exit(64);
    }
}

// fn get_line(line: usize, error_line: &String, line_num: i32) -> String {
//     if line_num == -1 {
//         return format!(
//             "{}\n",
//             Black.paint(format!(
//                 "{} | {}",
//                 ("0".to_string() + &(line - 1).to_string()),
//                 &error_line[find_nth_index(&error_line, '\n', line - 2).unwrap() + 1
//                     ..find_nth_index(&error_line, '\n', line - 1).unwrap()]
//             ))
//         );
//     } else if line_num == 0 {
//         return format!(
//             "{} | {}",
//             ("0".to_string() + &line.to_string()),
//             &error_line[find_nth_index(&error_line, '\n', line - 1).unwrap() + 1
//                 ..find_nth_index(&error_line, '\n', line).unwrap()]
//         );
//     } else if line_num == 1 {
//         if (find_nth_index(&error_line, '\n', line).unwrap() + 1)
//             != find_nth_index(&error_line, '\n', line + 1).unwrap() + 1
//         {
//             return format!(
//                 "{}\n",
//                 Black.paint(format!(
//                     "{} | {}",
//                     ("0".to_string() + &(line + 1).to_string()),
//                     &error_line[find_nth_index(&error_line, '\n', line).unwrap() + 1
//                         ..find_nth_index(&error_line, '\n', line + 1).unwrap()]
//                 ))
//             );
//         } else {
//             return " ".to_string();
//         }
//     } else {
//         return " ".to_string();
//     }
// }

// fn create_string_with_spaces(x: usize, c: char) -> String {
//     let mut s = String::new();
//     for _ in 0..x + 3 {
//         s.push(' ');
//     }
//     s.push(c);
//     s
// }

// fn find_nth_index(s: &str, c: char, n: usize) -> Option<usize> {
//     let mut iter = s.char_indices().filter(|&(_, ch)| ch == c).skip(n - 1);
//     match iter.next() {
//         Some((i, _)) => Some(i),
//         None => Some(s.len()),
//     }
// }
