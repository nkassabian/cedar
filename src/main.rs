mod error;
mod token;
use error::*;
mod scanner;
mod token_type;

use scanner::*;
use std::env::args;
use std::io::{self, stdout, BufRead, Write};

fn main() {
    let args: Vec<String> = args().collect();
    //println!("{}", args[1]);
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    if run(buf, path.to_string()).is_err() {
        // Ignore: error was already reported
        std::process::exit(65);
    }

    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line, "Prompt Error".to_string());
        } else {
            break;
        }
        print!("> ");
        let _ = stdout().flush();
    }
}

fn run(source: String, file_name: String) -> Result<(), KayLanError> {
    let mut scanner = Scanner::new(source.chars().collect(), file_name);
    let tokens = scanner.scan_tokens();

    for token in tokens? {
        println!("{:?}", token);
    }
    Ok(())
}

// struct KayLan {
//     had_error: bool,
// }
