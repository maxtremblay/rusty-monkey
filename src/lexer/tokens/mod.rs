pub mod kind;
pub use kind::TokenKind;

pub struct Token {
    kind: TokenKind,
    literal: String,
}
