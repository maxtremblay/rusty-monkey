use super::ast_nodes;
use super::{Expression, Parser, Statement};
use super::{InfixParsingFunction, PrefixParsingFunction};
use crate::lexer::tokens::kind::{Delimiter, Keyword, Operator};
use crate::lexer::{Token, TokenIter, TokenKind};
use std::collections::HashMap;
use std::iter::Peekable;

use ParsingError::*;

pub struct StatementIter<'a> {
    tokens: Peekable<TokenIter<'a>>,
    infix_parsing_functions: &'a HashMap<TokenKind, InfixParsingFunction>,
    prefix_parsing_functions: &'a HashMap<TokenKind, PrefixParsingFunction>,
}

impl<'a> From<&'a Parser> for StatementIter<'a> {
    fn from(parser: &'a Parser) -> Self {
        Self {
            tokens: parser.lexer.tokens().peekable(),
            infix_parsing_functions: &parser.infix_parsing_functions,
            prefix_parsing_functions: &parser.prefix_parsing_functions,
        }
    }
}

impl<'a> Iterator for StatementIter<'a> {
    type Item = ParsingResult<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|token| match token.kind {
            TokenKind::Keyword(Keyword::Let) => self.parse_let_statement(token),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_statement(token),
            _ => Err(NotAStatement),
        })
    }
}

impl<'a> StatementIter<'a> {
    fn parse_let_statement(&mut self, token: Token) -> ParsingResult<Statement> {
        let identifier = self.parse_next_identifier()?;
        self.skip_next_token_if(TokenKind::Operator(Operator::Assignment))?;
        let expression =
            self.parse_next_expression_until(TokenKind::Delimiter(Delimiter::SemiColon))?;
        self.skip_semi_colon()?;
        let statement = ast_nodes::LetStatement {
            token,
            identifier,
            expression,
        };
        Ok(Statement::Let(statement))
    }

    fn parse_return_statement(&mut self, token: Token) -> ParsingResult<Statement> {
        let expression =
            self.parse_next_expression_until(TokenKind::Delimiter(Delimiter::SemiColon))?;
        self.skip_semi_colon()?;
        let statement = ast_nodes::ReturnStatement { token, expression };
        Ok(Statement::Return(statement))
    }

    fn parse_next_identifier(&mut self) -> ParsingResult<ast_nodes::Identifier> {
        let token = self.peek_next_token()?;
        if let TokenKind::Identifier = token.kind {
            let token = self.tokens.next().unwrap();
            Ok(ast_nodes::Identifier { token })
        } else {
            self.skip_until(TokenKind::Delimiter(Delimiter::SemiColon))?;
            Err(MissingIdentifier)
        }
    }

    fn skip_next_token_if(&mut self, kind: TokenKind) -> ParsingResult<()> {
        let token = self.peek_next_token()?;
        if token.kind == kind {
            self.tokens.next();
            Ok(())
        } else {
            Err(MissingToken)
        }
    }

    fn skip_semi_colon(&mut self) -> ParsingResult<()> {
        self.skip_next_token_if(TokenKind::Delimiter(Delimiter::SemiColon))
    }

    fn parse_next_expression_until(&mut self, kind: TokenKind) -> ParsingResult<Expression> {
        while let Some(token) = self.tokens.peek() {
            if token.kind == kind {
                return Ok(Expression::Empty);
            }
            self.tokens.next();
        }
        Err(ReachEndOfFile)
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
    MissingToken,
    MissingIdentifier,
    NotAStatement,
    ReachEndOfFile,
}
