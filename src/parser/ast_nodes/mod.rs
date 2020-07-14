use crate::lexer::Token;

mod macros;

#[macro_use]
use crate::create_node;

pub trait Node {
    fn token_literal(&self) -> &str;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Empty,
}

create_node!(LetStatement with
    identifier: Identifier,
    expression: Expression,
);

create_node!(ReturnStatement with expression: Expression);
create_node!(Identifier);
