use super::{Lexer, Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub struct TokenIter<'a> {
    input: Peekable<Chars<'a>>,
    literal: Option<String>,
}

impl<'a> From<&'a Lexer> for TokenIter<'a> {
    fn from(lexer: &'a Lexer) -> Self {
        Self {
            input: lexer.input.chars().peekable(),
            literal: None,
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|character| match character {
            c if Self::is_delimiter(c) => Token::from_delimiter(c),
            c if Self::is_operator(c) => Token::from_operator(c.to_string()),
            invalid_literal => Token::invalid_with_literal(invalid_literal.to_string()),
        })
    }
}

impl<'a> TokenIter<'a> {
    fn is_delimiter(character: char) -> bool {
        vec![';', ',', '(', ')', '{', '}'].contains(&character)
    }

    fn is_operator(character: char) -> bool {
        vec!['=', '+'].contains(&character)
    }
}
