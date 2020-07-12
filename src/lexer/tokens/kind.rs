#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TokenKind {
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

    // Specials
    Invalid,
    EndOfFile,
}
