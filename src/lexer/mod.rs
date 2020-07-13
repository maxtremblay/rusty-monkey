pub mod tokens;
pub use tokens::{Token, TokenKind};

pub mod token_iter;
pub use token_iter::TokenIter;

use std::collections::HashSet;

#[cfg(test)]
mod test;

pub struct Lexer {
    input: String,
    delimiters: HashSet<char>,
    keywords: HashSet<String>,
}

impl From<String> for Lexer {
    fn from(input: String) -> Self {
        Self {
            input,
            delimiters: Token::delimiters(),
            keywords: Token::keywords(),
        }
    }
}

impl Lexer {
    pub fn tokens(&self) -> TokenIter {
        TokenIter::from(self)
    }
}
