mod lexer;
mod token;
use crate::lexer::Lexer;

fn main() {
    let input = std::fs::read_to_string("src/main.mky").expect("Error reading file.");
    let mut lexer = Lexer::new(input);

    println!("{:?}", lexer.read());
}
