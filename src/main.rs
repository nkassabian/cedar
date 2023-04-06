mod ast_printer;
mod error;
mod expr;
mod interpreter;
mod stmt;
mod token;
use error::*;
mod errors {
    pub(crate) mod lexer_error;
    pub(crate) mod syntax_error;
}
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

    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf, path.to_string()).is_err() {
            // Ignore: error was already reported
            std::process::exit(65);
        }

        Ok(())
    }

    fn run_prompt(&mut self) {
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

    fn run(&mut self, source: String, file_name: String) -> Result<(), CDLexerError> {
        let mut scanner = Scanner::new(source.chars().collect(), file_name);
        let tokens = scanner.scan_tokens();

        // for token in tokens? {
        //     println!("{:?}", token);
        // }

        let mut parser = Parser::new(tokens.unwrap().clone());
        //let mut statements: Vec<Stmt> = parser.parse().unwrap();
        match parser.parse() {
            Ok(statements) => {
                self.interpreter.interpret(&statements);
            }
            Err(error) => error.report(),
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut cedar = Cedar::new();
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
