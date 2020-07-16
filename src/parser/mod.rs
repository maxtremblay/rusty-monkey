pub mod ast_nodes;
pub use ast_nodes::{Expression, Node, Statement};

pub mod statement_iter;
pub use statement_iter::{ParsingError, ParsingResult, StatementIter};

pub mod pratt;
use pratt::{InfixParsingFunction, PrefixParsingFunction};

use crate::lexer::{Lexer, TokenKind};
use std::collections::HashMap;

pub struct Parser {
    lexer: Lexer,
    infix_parsing_functions: HashMap<TokenKind, InfixParsingFunction>,
    prefix_parsing_functions: HashMap<TokenKind, PrefixParsingFunction>,
}

impl From<Lexer> for Parser {
    fn from(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            infix_parsing_functions: Self::infix_parsing_functions(),
            prefix_parsing_functions: Self::prefix_parsing_functions(),
        }
    }
}

impl Parser {
    fn infix_parsing_functions() -> HashMap<TokenKind, InfixParsingFunction> {
        HashMap::with_capacity(0)
    }

    fn prefix_parsing_functions() -> HashMap<TokenKind, PrefixParsingFunction> {
        HashMap::with_capacity(0)
    }

    pub fn statements(&self) -> StatementIter {
        StatementIter::from(self)
    }
}

#[cfg(test)]
mod test;
