pub mod ast_nodes;
pub use ast_nodes::{Expression, Node, Statement};

pub mod statement_iter;
pub use statement_iter::{ParsingError, ParsingResult, StatementIter};

pub mod parsing_functions;
pub use parsing_functions::ParsingFunctions;

pub mod precedence;
pub use precedence::Precedence;

use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
    parsing_functions: ParsingFunctions,
}

impl From<Lexer> for Parser {
    fn from(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            parsing_functions: ParsingFunctions::new(),
        }
    }
}

impl Parser {
    pub fn statements(&self) -> StatementIter {
        StatementIter::from(self)
    }
}

#[cfg(test)]
mod test;
