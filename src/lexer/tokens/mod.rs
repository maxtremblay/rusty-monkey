pub mod kind;
pub use kind::TokenKind;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub(super) kind: TokenKind,
    pub(super) literal: String,
}

impl Token {
    pub fn from_delimiter(delimiter: char) -> Self {
        Self {
            kind: TokenKind::from_delimiter(delimiter),
            literal: delimiter.to_string(),
        }
    }

    pub fn from_identifier(identifier: String) -> Self {
        Self {
            kind: TokenKind::Identifier,
            literal: identifier,
        }
    }

    pub fn from_keyword(keyword: String) -> Self {
        Self {
            kind: TokenKind::from_keyword(&keyword),
            literal: keyword,
        }
    }

    pub fn from_number(number: String) -> Self {
        Self {
            kind: TokenKind::Number,
            literal: number,
        }
    }

    pub fn from_operator(operator: String) -> Self {
        Self {
            kind: TokenKind::from_operator(&operator),
            literal: operator,
        }
    }

    pub fn invalid_with_literal(literal: String) -> Self {
        Self {
            kind: TokenKind::Invalid,
            literal,
        }
    }

    pub fn delimiters() -> HashSet<char> {
        kind::Delimiter::all()
    }

    pub fn keywords() -> HashSet<String> {
        kind::Keyword::all()
    }

    pub fn operators() -> HashSet<String> {
        kind::Operator::all()
    }
}
