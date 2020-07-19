use super::ast_nodes::Identifier;
use super::{Expression, StatementIter};
use super::{ParsingError, ParsingResult};
use crate::lexer::TokenKind;

pub type InfixParsingFunction = fn(&mut StatementIter, Expression) -> ParsingResult<Expression>;
pub type PrefixParsingFunction = fn(&mut StatementIter) -> ParsingResult<Expression>;

pub fn infix_parsing_function_of() -> Option<InfixParsingFunction> {
    todo!()
}

pub fn prefix_parsing_function_of(token_kind: TokenKind) -> Option<PrefixParsingFunction> {
    match token_kind {
        TokenKind::Identifier => Some(identifier_prefix_parsing_function),
        _ => None,
    }
}

pub fn identifier_prefix_parsing_function(
    statements: &mut StatementIter,
) -> ParsingResult<Expression> {
    let token = statements
        .peek_current_token()
        .as_ref()
        .ok_or(ParsingError::InvalidExpression)?;
    Ok(Expression::Identifier(Identifier {
        token: token.clone(),
    }))
}
