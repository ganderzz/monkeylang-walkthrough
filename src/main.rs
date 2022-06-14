mod lexer;
mod repl;
mod token;
use std::env;

use crate::lexer::Lexer;

fn main() {
    let mut args = env::args();

    if args.len() > 1 && args.nth(1).unwrap() == "repl" {
        repl::repl::run();
        return;
    }

    let input = std::fs::read_to_string("src/main.mky").expect("Error reading file.");
    let mut lexer = Lexer::new(input.as_str());

    println!("{:?}", lexer.read());
}
