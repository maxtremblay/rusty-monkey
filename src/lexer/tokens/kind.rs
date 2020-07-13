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
    Else,
    False,
    Function,
    If,
    Let,
    Return,
    True,
}

impl Keyword {
    pub fn all() -> HashSet<String> {
        let mut keywords = HashSet::with_capacity(7);
        keywords.insert(String::from("else"));
        keywords.insert(String::from("false"));
        keywords.insert(String::from("fn"));
        keywords.insert(String::from("if"));
        keywords.insert(String::from("let"));
        keywords.insert(String::from("return"));
        keywords.insert(String::from("true"));
        keywords
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Operator {
    Assignment,
    Asterix,
    Bang,
    GreaterThan,
    LessThan,
    Minus,
    Plus,
    Slash,
}

impl Operator {
    pub fn all() -> HashSet<String> {
        let mut operators = HashSet::with_capacity(8);
        operators.insert(String::from("="));
        operators.insert(String::from("*"));
        operators.insert(String::from("!"));
        operators.insert(String::from(">"));
        operators.insert(String::from("<"));
        operators.insert(String::from("-"));
        operators.insert(String::from("+"));
        operators.insert(String::from("/"));
        operators
    }
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
            "else" => TokenKind::Keyword(Keyword::Else),
            "false" => TokenKind::Keyword(Keyword::False),
            "if" => TokenKind::Keyword(Keyword::If),
            "let" => TokenKind::Keyword(Keyword::Let),
            "return" => TokenKind::Keyword(Keyword::Return),
            "true" => TokenKind::Keyword(Keyword::True),
            _ => TokenKind::Invalid,
        }
    }

    pub fn from_operator(operator: &str) -> Self {
        match operator {
            "=" => TokenKind::Operator(Operator::Assignment),
            "*" => TokenKind::Operator(Operator::Asterix),
            "!" => TokenKind::Operator(Operator::Bang),
            ">" => TokenKind::Operator(Operator::GreaterThan),
            "<" => TokenKind::Operator(Operator::LessThan),
            "-" => TokenKind::Operator(Operator::Minus),
            "+" => TokenKind::Operator(Operator::Plus),
            "/" => TokenKind::Operator(Operator::Slash),
            _ => TokenKind::Invalid,
        }
    }
}
