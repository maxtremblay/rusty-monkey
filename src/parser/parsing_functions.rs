use super::ast_nodes::{Identifier, IntegerLiteral};
use super::{Expression, StatementIter};
use super::{ParsingError, ParsingResult};
use crate::lexer::TokenKind;
use std::collections::HashMap;

pub type InfixParsingFunction =
    Box<dyn Fn(&mut StatementIter, Expression) -> ParsingResult<Expression>>;
pub type PrefixParsingFunction = Box<dyn Fn(&mut StatementIter) -> ParsingResult<Expression>>;

pub struct ParsingFunctions {
    pub infix: HashMap<TokenKind, InfixParsingFunction>,
    pub prefix: HashMap<TokenKind, PrefixParsingFunction>,
}

impl ParsingFunctions {
    pub fn new() -> Self {
        Self {
            infix: Self::generate_infix_functions(),
            prefix: Self::generate_prefix_functions(),
        }
    }

    fn generate_infix_functions() -> HashMap<TokenKind, InfixParsingFunction> {
        HashMap::with_capacity(0)
    }

    fn generate_prefix_functions() -> HashMap<TokenKind, PrefixParsingFunction> {
        let mut functions = HashMap::with_capacity(2);
        functions.insert(TokenKind::Identifier, Self::identifier_prefix());
        functions.insert(TokenKind::Number, Self::number_prefix());
        functions
    }

    fn identifier_prefix() -> PrefixParsingFunction {
        Box::new(|statements: &mut StatementIter| {
            statements
                .peek_current_token()
                .as_ref()
                .ok_or(ParsingError::InvalidExpression)
                .map(|token| {
                    Expression::Identifier(Identifier {
                        token: token.clone(),
                    })
                })
        })
    }

    fn number_prefix() -> PrefixParsingFunction {
        Box::new(|statements: &mut StatementIter| {
            statements
                .peek_current_token()
                .as_ref()
                .ok_or(ParsingError::InvalidExpression)
                .and_then(|token| {
                    i64::from_str_radix(&token.literal, 10)
                        .map_err(|_| ParsingError::InvalidExpression)
                        .map(|value| {
                            Expression::IntegerLiteral(IntegerLiteral {
                                token: token.clone(),
                                value,
                            })
                        })
                })
        })
    }
}
