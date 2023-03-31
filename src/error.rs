use ansi_term::Colour::{Blue, Purple, Red, Yellow};

#[derive(Debug)]
pub struct KayLanError {
    line: usize,
    message: String,
    column: usize,
    main_message: String,
    file_name: String,
}

impl KayLanError {
    pub fn error(
        line: usize,
        column: usize,
        main_message: String,
        message: String,
        file_name: String,
    ) -> KayLanError {
        KayLanError {
            line,
            message,
            column,
            main_message,
            file_name,
        }
    }

    // [src/test.ql->1:20::Unexpected token found. of type: ;
    // 0 |
    // 1 | have shipSpeedX := 0;
    //                       ^^^ // this is the position of the token in the line above
    // 2 |
    pub fn report(self) {
        eprint!(
            "\n\n[{}]->{}::{}\n\n",
            Yellow.bold().paint(self.file_name),
            format!(
                "{}:{}",
                Red.bold().paint(self.line.to_string()),
                Purple.bold().paint(self.column.to_string())
            ),
            format!("{}, {}", Blue.bold().paint(self.main_message), self.message),
        )
    }
}

// fn create_string_with_spaces(x: usize, c: char) -> String {
//     let mut s = String::new();
//     for _ in 0..x + 3 {
//         s.push(' ');
//     }
//     s.push(c);
//     s
// }
