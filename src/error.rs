use ansi_term::Colour::{Black, Blue, Purple, Red, Yellow};

#[derive(Debug)]
pub struct KayLanError {
    line: usize,
    message: String,
    column: usize,
    main_message: String,
    file_name: String,
    source_toks: Vec<char>,
}

impl KayLanError {
    pub fn error(
        line: usize,
        column: usize,
        main_message: String,
        message: String,
        file_name: String,
        source_toks: Vec<char>,
    ) -> KayLanError {
        KayLanError {
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
        // println!("{:?}", find_nth_index(&line, '\n', self.line).unwrap());
        // println!("{:?}", find_nth_index(&line, '\n', self.line + 1).unwrap());
        // println!(
        //     "{:?}",
        //     &line[find_nth_index(&line, '\n', self.line - 1).unwrap() + 1
        //         ..find_nth_index(&line, '\n', self.line).unwrap()]
        // );
        //

        if self.line < 10 {
            println!(
                "{}\n",
                Black.paint(format!(
                    "{} | {}",
                    ("0".to_string() + &(&self.line - 1).to_string()),
                    &line[find_nth_index(&line, '\n', self.line - 2).unwrap() + 1
                        ..find_nth_index(&line, '\n', self.line - 1).unwrap()]
                ))
            );
            println!(
                "{} | {}",
                ("0".to_string() + &self.line.to_string()),
                &line[find_nth_index(&line, '\n', self.line - 1).unwrap() + 1
                    ..find_nth_index(&line, '\n', self.line).unwrap()]
            );
            println!(
                "{}",
                Red.bold()
                    .paint(create_string_with_spaces(self.column + 2, '^'))
            );

            if (find_nth_index(&line, '\n', self.line).unwrap() + 1)
                != find_nth_index(&line, '\n', self.line + 1).unwrap() + 1
            {
                println!(
                    "{}",
                    Black.paint(format!(
                        "{} | {}",
                        ("0".to_string() + &(&self.line + 1).to_string()),
                        &line[find_nth_index(&line, '\n', self.line).unwrap() + 1
                            ..find_nth_index(&line, '\n', self.line + 1).unwrap()]
                    ))
                );
            }
        }

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
