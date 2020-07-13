use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TokenKind {
    Delimiter(Delimiter),
    Identifier,
    Invalid,
    Keyword(Keyword),
    Number,
    Operator(Operator),
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

impl Delimiter {
    pub fn all() -> HashSet<char> {
        let mut delimiters = HashSet::with_capacity(6);
        delimiters.insert(',');
        delimiters.insert(';');
        delimiters.insert('(');
        delimiters.insert(')');
        delimiters.insert('{');
        delimiters.insert('}');
        delimiters
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Function,
    Let,
}

impl Keyword {
    pub fn all() -> HashSet<String> {
        let mut keywords = HashSet::with_capacity(2);
        keywords.insert(String::from("fn"));
        keywords.insert(String::from("let"));
        keywords
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Operator {
    Assignment,
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

    pub fn from_keyword(keyword: &str) -> Self {
        match keyword {
            "fn" => TokenKind::Keyword(Keyword::Function),
            "let" => TokenKind::Keyword(Keyword::Let),
            _ => TokenKind::Invalid,
        }
    }

    pub fn from_operator(operator: &str) -> Self {
        match operator {
            "=" => TokenKind::Operator(Operator::Assignment),
            "+" => TokenKind::Operator(Operator::Plus),
            _ => TokenKind::Invalid,
        }
    }
}
