pub mod ast_nodes;
pub use ast_nodes::{Expression, Node, Statement};

pub mod statement_iter;
pub use statement_iter::{ParsingError, ParsingResult, StatementIter};

use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
}

impl From<Lexer> for Parser {
    fn from(lexer: Lexer) -> Parser {
        Parser { lexer }
    }
}

impl Parser {
    pub fn statements(&self) -> StatementIter {
        StatementIter::from(self)
    }
}

#[cfg(test)]
mod test;
