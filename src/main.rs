mod ast_printer;
mod error;
mod expr;
mod interpreter;
mod token;
use error::*;
mod object;
mod parser;
mod scanner;
mod token_type;
// mod expr;

use interpreter::*;
use parser::Parser;
use scanner::*;
use std::env::args;
use std::io::{self, stdout, BufRead, Write};

struct Cedar {
    interpreter: Interpreter,
}

impl Cedar {
    pub fn new() -> Cedar {
        Cedar {
            interpreter: Interpreter {},
        }
    }

    fn run_file(&self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf, path.to_string()).is_err() {
            // Ignore: error was already reported
            std::process::exit(65);
        }

        Ok(())
    }

    fn run_prompt(&self) {
        let stdin = io::stdin();
        print!("> ");
        let _ = stdout().flush();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                let _ = self.run(line, "Prompt Error".to_string());
            } else {
                break;
            }
            print!("> ");
            let _ = stdout().flush();
        }
    }

    fn run(&self, source: String, file_name: String) -> Result<(), CDLexerError> {
        let mut scanner = Scanner::new(source.chars().collect(), file_name);
        let tokens = scanner.scan_tokens();

        // for token in tokens? {
        //     println!("{:?}", token);
        // }

        let mut parser = Parser::new(tokens.unwrap().clone());
        match parser.parse() {
            Ok(expr) => {
                self.interpreter.interpret(&expr);
            }
            Err(error) => {
                eprint!("{:?}", error);
            }
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let cedar = Cedar::new();
    //println!("{}", args[1]);
    match args.len() {
        1 => cedar.run_prompt(),
        2 => cedar.run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

// struct KayLan {
//     had_error: bool,
// }
