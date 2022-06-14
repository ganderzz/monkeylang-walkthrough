use std::io::stdin;

use crate::lexer::Lexer;

pub fn run() {
    println!("Entering Monkey REPL.");
    println!("Enter 'q' to quit.");

    loop {
        print!(">> ");

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Could not read from stdin.");

        match input.to_lowercase().as_str() {
            "q" => return,
            rest => {
                let mut lexer = Lexer::new(rest);

                println!("{:?}", lexer.read());
            }
        }
    }
}
