pub mod kind;
pub use kind::TokenKind;

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
}
