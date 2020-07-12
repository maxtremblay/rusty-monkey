#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TokenKind {
    Invalid,
    EndOfFile,

    // Identifiers and literals
    Identifier,
    Integer,

    // Operators
    Assignment,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
}
