use crate::lexer::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
}

//impl<'a> Node for Statement<'a> {
//fn token_literal(&self) -> &str {
//match self {
//Self::Let(statement) => statement.token_literal(),
//}
//}
//}

#[derive(Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub(crate) identifier: Identifier,
    pub(crate) expression: Expression,
    pub(crate) token: Token,
}

impl<'a> Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    token: Token,
    value: String,
}

impl From<Token> for Identifier {
    fn from(token: Token) -> Self {
        let value = token.literal.clone();
        Self { token, value }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}
