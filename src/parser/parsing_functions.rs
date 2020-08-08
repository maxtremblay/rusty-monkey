use super::ast_nodes::{Identifier, Number, PrefixOperator};
use super::Precedence;
use super::{Expression, StatementIter};
use super::{ParsingError, ParsingResult};
use crate::lexer::tokens::kind::Operator;
use crate::lexer::{Token, TokenKind};
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
        functions.insert(TokenKind::Identifier, Self::identifier_prefix_function());
        functions.insert(TokenKind::Number, Self::number_prefix_function());
        functions.insert(
            TokenKind::Operator(Operator::Bang),
            Self::prefix_operator_prefix_function(),
        );
        functions.insert(
            TokenKind::Operator(Operator::Minus),
            Self::prefix_operator_prefix_function(),
        );
        functions
    }

    fn identifier_prefix_function() -> PrefixParsingFunction {
        Box::new(|statements: &mut StatementIter| {
            Self::token_from_statements(statements).map(|token| {
                Expression::Identifier(Identifier {
                    token: token.clone(),
                })
            })
        })
    }

    fn number_prefix_function() -> PrefixParsingFunction {
        Box::new(|statements: &mut StatementIter| {
            Self::token_from_statements(statements).and_then(|token| {
                i64::from_str_radix(&token.literal, 10)
                    .map_err(|_| ParsingError::InvalidExpression)
                    .map(|value| {
                        Expression::Number(Number {
                            token: token.clone(),
                            value,
                        })
                    })
            })
        })
    }

    fn prefix_operator_prefix_function() -> PrefixParsingFunction {
        Box::new(|statements: &mut StatementIter| {
            let token = Self::token_from_statements(statements)?;
            let token = token.clone();
            let operator = token.literal.clone();
            statements.advance_token();
            let right_expression = statements.parse_expression(Precedence::Prefix)?;
            Ok(Expression::PrefixOperator(PrefixOperator {
                token,
                operator,
                right_expression: Box::new(right_expression),
            }))
        })
    }

    fn token_from_statements<'a>(statements: &'a mut StatementIter) -> ParsingResult<&'a Token> {
        statements
            .peek_current_token()
            .as_ref()
            .ok_or(ParsingError::InvalidExpression)
    }
}
