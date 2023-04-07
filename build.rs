mod generate_ast;
use generate_ast::*;

fn main() -> () {
    generate_ast(&"src".to_string()).unwrap();
}
