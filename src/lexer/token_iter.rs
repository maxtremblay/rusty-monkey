use super::{Lexer, Token};
use std::collections::HashSet;
use std::iter::Peekable;
use std::str::Chars;

pub struct TokenIter<'a> {
    input: Peekable<Chars<'a>>,
    delimiters: &'a HashSet<char>,
    keywords: &'a HashSet<String>,
    operators: &'a HashSet<String>,
}

impl<'a> From<&'a Lexer> for TokenIter<'a> {
    fn from(lexer: &'a Lexer) -> Self {
        Self {
            input: lexer.input.chars().peekable(),
            delimiters: &lexer.delimiters,
            keywords: &lexer.keywords,
            operators: &lexer.operators,
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        self.input.next().map(|character| match character {
            c if self.is_delimiter(c) => Token::from_delimiter(c),
            c if self.is_operator(&c.to_string()) => self.read_operator_starting_with(c),
            c if Self::is_valid_in_identifiers_and_keywords(c) => {
                self.read_identifier_or_keyword_starting_with(c)
            }
            c if c.is_ascii_digit() => self.read_number_starting_with(c),
            invalid_literal => Token::invalid_with_literal(invalid_literal.to_string()),
        })
    }
}

impl<'a> TokenIter<'a> {
    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.input.peek() {
            if !c.is_whitespace() {
                return;
            }
            self.input.next();
        }
    }

    fn is_delimiter(&self, c: char) -> bool {
        self.delimiters.contains(&c)
    }

    fn is_keyword(&self, literal: &str) -> bool {
        self.keywords.contains(literal)
    }

    fn is_operator(&self, literal: &str) -> bool {
        self.operators.contains(literal)
    }

    fn read_operator_starting_with(&mut self, start: char) -> Token {
        if let Some(c) = self.input.peek() {
            let operator = format!("{}{}", start, c);
            if self.is_operator(&operator) {
                self.input.next();
                return Token::from_operator(operator);
            }
        }
        Token::from_operator(start.to_string())
    }

    fn is_valid_in_identifiers_and_keywords(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn read_identifier_or_keyword_starting_with(&mut self, start: char) -> Token {
        let mut literal = start.to_string();
        while let Some(c) = self.input.peek() {
            if !Self::is_valid_in_identifiers_and_keywords(*c) {
                break;
            }
            literal.push(self.input.next().unwrap());
        }
        if self.is_keyword(&literal) {
            Token::from_keyword(literal)
        } else {
            Token::from_identifier(literal)
        }
    }

    fn read_number_starting_with(&mut self, start: char) -> Token {
        let mut number = start.to_string();
        while let Some(c) = self.input.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            number.push(self.input.next().unwrap());
        }
        Token::from_number(number)
    }
}
