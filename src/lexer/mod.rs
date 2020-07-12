pub mod tokens;
pub use tokens::{Token, TokenKind};

pub mod token_iter;
pub use token_iter::TokenIter;

#[cfg(test)]
mod test;

pub struct Lexer {
    input: String,
}

impl From<String> for Lexer {
    fn from(input: String) -> Self {
        Self { input }
    }
}

impl Lexer {
    pub fn tokens(&self) -> TokenIter {
        TokenIter::from(self)
    }
}
