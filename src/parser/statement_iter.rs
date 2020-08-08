use super::ast_nodes;
use super::parsing_functions::ParsingFunctions;
use super::{Expression, Parser, Precedence, Statement};
use crate::lexer::tokens::kind::{Delimiter, Keyword, Operator};
use crate::lexer::{Token, TokenIter, TokenKind};
use std::iter::Peekable;

use ParsingError::*;

pub struct StatementIter<'a> {
    tokens: Peekable<TokenIter<'a>>,
    parsing_functions: &'a ParsingFunctions,
    current: Option<Token>,
}

impl<'a> From<&'a Parser> for StatementIter<'a> {
    fn from(parser: &'a Parser) -> Self {
        Self {
            tokens: parser.lexer.tokens().peekable(),
            parsing_functions: &parser.parsing_functions,
            current: None,
        }
    }
}

impl<'a> Iterator for StatementIter<'a> {
    type Item = ParsingResult<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|token| {
            self.current = Some(token);
            match self.current.as_ref().unwrap().kind {
                TokenKind::Keyword(Keyword::Let) => self.parse_let_statement(),
                TokenKind::Keyword(Keyword::Return) => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            }
        })
    }
}

impl<'a> StatementIter<'a> {
    fn parse_let_statement(&mut self) -> ParsingResult<Statement> {
        let identifier = self.parse_next_identifier()?;
        self.skip_next_token_if(TokenKind::Operator(Operator::Assignment))?;
        let expression =
            self.parse_next_expression_until(TokenKind::Delimiter(Delimiter::SemiColon))?;
        self.skip_semi_colon()?;
        let statement = ast_nodes::LetStatement {
            token: self.take_current_token(),
            identifier,
            expression,
        };
        Ok(Statement::Let(statement))
    }

    fn parse_return_statement(&mut self) -> ParsingResult<Statement> {
        let expression =
            self.parse_next_expression_until(TokenKind::Delimiter(Delimiter::SemiColon))?;
        self.skip_semi_colon()?;
        let statement = ast_nodes::ReturnStatement {
            token: self.take_current_token(),
            expression,
        };
        Ok(Statement::Return(statement))
    }

    fn parse_expression_statement(&mut self) -> ParsingResult<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.skip_semi_colon()?;
        let statement = ast_nodes::ExpressionStatement {
            expression,
            token: self.take_current_token(),
        };
        Ok(Statement::Expression(statement))
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

    pub fn peek_current_token(&self) -> &Option<Token> {
        &self.current
    }

    fn take_current_token(&mut self) -> Token {
        self.current.take().unwrap()
    }

    fn skip_until(&mut self, kind: TokenKind) -> ParsingResult<()> {
        while let Some(token) = self.tokens.next() {
            if token.kind == kind {
                return Ok(());
            }
        }
        Err(ReachEndOfFile)
    }

    pub(super) fn advance_token(&mut self) {
        self.current = self.tokens.next();
    }

    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> ParsingResult<Expression> {
        self.peek_current_token()
            .as_ref()
            .and_then(|token| self.parsing_functions.prefix.get(&token.kind))
            .ok_or(ParsingError::InvalidExpression)
            .and_then(|prefix| prefix(self))
    }
}

pub type ParsingResult<T> = Result<T, ParsingError>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ParsingError {
    InvalidExpression,
    MissingToken,
    MissingIdentifier,
    NotAStatement,
    ReachEndOfFile,
}
