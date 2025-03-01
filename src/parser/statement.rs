use crate::parser::ast::{Annotation, Expr, Statement};
use crate::parser::{Parser, ParserError};
use crate::span::{Span, Spanned};
use crate::tokens::TokenKind;

type StatementResult = Result<Spanned<Statement>, ParserError>;

impl<'a> Parser<'a> {
    /// Parses a statement.
    pub fn parse_statement(&mut self) -> StatementResult {
        let peeked = self.peek();

        todo!()
    }
}
