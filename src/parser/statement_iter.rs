use super::ast_nodes;
use super::{Expression, Parser, Statement};
use crate::lexer::tokens::kind::{Delimiter, Keyword, Operator};
use crate::lexer::{Token, TokenIter, TokenKind};
use std::iter::Peekable;

use ParsingError::*;

#[derive(Debug)]
pub struct StatementIter<'a> {
    tokens: Peekable<TokenIter<'a>>,
}

impl<'a> From<&'a Parser> for StatementIter<'a> {
    fn from(parser: &'a Parser) -> Self {
        Self {
            tokens: parser.lexer.tokens().peekable(),
        }
    }
}

impl<'a> Iterator for StatementIter<'a> {
    type Item = ParsingResult<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|token| match token.kind {
            TokenKind::Keyword(Keyword::Let) => self.parse_let_statement(token),
            _ => Err(NotAStatement),
        })
    }
}

impl<'a> StatementIter<'a> {
    fn parse_let_statement(&mut self, let_token: Token) -> ParsingResult<Statement> {
        let identifier = self.parse_next_identifier()?;
        self.skip_next_token_if(TokenKind::Operator(Operator::Assignment))?;
        let expression = self.parse_next_expression()?;
        let statement = ast_nodes::LetStatement {
            token: let_token,
            identifier,
            expression,
        };
        Ok(Statement::Let(statement))
    }

    fn parse_next_identifier(&mut self) -> ParsingResult<ast_nodes::Identifier> {
        let token = self.peek_next_token()?;
        if let TokenKind::Identifier = token.kind {
            let token = self.tokens.next().unwrap();
            Ok(ast_nodes::Identifier::from(token))
        } else {
            self.skip_until(TokenKind::Delimiter(Delimiter::SemiColon))?;
            Err(MissingIdentifier)
        }
    }

    fn skip_next_token_if(&mut self, kind: TokenKind) -> ParsingResult<()> {
        let token = self.peek_next_token()?;
        if token.kind == kind {
            self.tokens.next();
        }
        Ok(())
    }

    fn parse_next_expression(&mut self) -> ParsingResult<Expression> {
        while let Some(token) = self.tokens.next() {
            if let TokenKind::Delimiter(Delimiter::SemiColon) = token.kind {
                return Ok(Expression::Empty);
            }
        }
        Err(InvalidExpression)
    }

    fn peek_next_token(&mut self) -> ParsingResult<&Token> {
        self.tokens.peek().ok_or(ReachEndOfFile)
    }

    fn skip_until(&mut self, kind: TokenKind) -> ParsingResult<()> {
        while let Some(token) = self.tokens.next() {
            if token.kind == kind {
                return Ok(());
            }
        }
        Err(ReachEndOfFile)
    }
}

pub type ParsingResult<T> = Result<T, ParsingError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ParsingError {
    MissingAssignement,
    MissingIdentifier,
    InvalidExpression,
    NotAStatement,
    ReachEndOfFile,
}
