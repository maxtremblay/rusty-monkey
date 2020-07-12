#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TokenKind {
    Delimiter(Delimiter),
    Identifier,
    Invalid,
    Keyword(Keyword),
    Literal(Literal),
    Operators(Operators),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Delimiter {
    Comma,
    SemiColon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Function,
    Let,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Literal {
    Integer,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Operators {
    Assignement,
    Plus,
}

impl TokenKind {
    pub fn from_delimiter(delimiter: char) -> Self {
        match delimiter {
            ',' => TokenKind::Delimiter(Delimiter::Comma),
            ';' => TokenKind::Delimiter(Delimiter::SemiColon),
            '(' => TokenKind::Delimiter(Delimiter::LeftParenthesis),
            ')' => TokenKind::Delimiter(Delimiter::RightParenthesis),
            '{' => TokenKind::Delimiter(Delimiter::LeftBrace),
            '}' => TokenKind::Delimiter(Delimiter::RightBrace),
            _ => TokenKind::Invalid,
        }
    }
}
