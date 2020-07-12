pub mod kind;
pub use kind::TokenKind;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub(super) kind: TokenKind,
    pub(super) literal: String,
}

impl Token {
    pub fn from_symbol(symbol: char) -> Self {
        match symbol {
            '=' => Self {
                kind: TokenKind::Assignment,
                literal: String::from("="),
            },
            '+' => Self {
                kind: TokenKind::Plus,
                literal: String::from("+"),
            },
            ';' => Self {
                kind: TokenKind::Semicolon,
                literal: String::from(";"),
            },
            ',' => Self {
                kind: TokenKind::Comma,
                literal: String::from(","),
            },
            '(' => Self {
                kind: TokenKind::LeftParenthesis,
                literal: String::from("("),
            },
            ')' => Self {
                kind: TokenKind::RightParenthesis,
                literal: String::from(")"),
            },
            '{' => Self {
                kind: TokenKind::LeftBrace,
                literal: String::from("{"),
            },
            '}' => Self {
                kind: TokenKind::RightBrace,
                literal: String::from("}"),
            },
            invalid_symbol => Self::invalid_with_literal(invalid_symbol.to_string()),
        }
    }

    pub fn invalid_with_literal(literal: String) -> Self {
        Self {
            kind: TokenKind::Invalid,
            literal,
        }
    }
}
