use crate::lexer::Lexer;
use crate::parser::{Parser, ParsingResult, Statement};
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
                Ok(statements) => Self::print_statements(statements),
                Err(error) => println!("error: {}", error),
            }
        }
    }

    fn read(&self) -> io::Result<Vec<ParsingResult<Statement>>> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let statements = Parser::from(Lexer::from(input)).statements().collect();
        Ok(statements)
    }

    fn print_prompt(&self) {
        print!(">> ");
        io::stdout().flush().unwrap();
    }

    fn print_statements(statements: Vec<ParsingResult<Statement>>) {
        for statement in statements {
            match statement {
                Ok(statement) => println!("{}", statement),
                Err(error) => println!("Error: {:?}", error),
            }
        }
    }
}
