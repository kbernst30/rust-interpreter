pub mod interpreter;

use interpreter::intr::Interpreter;
use interpreter::lexer::Lexer;
use interpreter::parser::Parser;

use std::env;
use std::fs;

fn main() {
    // TODO file might not be present, if so drop to REPL
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Running file {:?}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let program = contents.chars().peekable();
    let mut lexer = Lexer::new(program);
    let mut parser = Parser::new(&mut lexer);
    let mut interpreter = Interpreter::new(&mut parser);

    interpreter.interpret();
}
