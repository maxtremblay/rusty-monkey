use crate::lexer::{Lexer, Token};
use std::io;
use std::io::Write;

pub struct ReadEvalPrintLoop;

impl ReadEvalPrintLoop {
    pub fn start() -> Self {
        println!("Welcome! This is the Monkey programming language.");
        println!("Please type in some commands.");
        Self
    }

    pub fn run(&self) {
        loop {
            self.print_prompt();
            match self.read() {
                Ok(tokens) => Self::print_tokens(tokens),
                Err(error) => println!("error: {}", error),
            }
        }
    }

    fn read(&self) -> io::Result<Vec<Token>> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let tokens = Lexer::from(input).tokens().collect();
        Ok(tokens)
    }

    fn print_prompt(&self) {
        print!(">> ");
        io::stdout().flush().unwrap();
    }

    fn print_tokens(tokens: Vec<Token>) {
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
