use crate::parser::ast::{Annotation, BinOpKind, Expr, Statement, UnaryOpKind};
use crate::parser::{Parser, ParserError};
use crate::span::{Span, Spanned};
use crate::tokens::TokenKind;

use super::ast::LiteralKind;

type ExprResult = Result<Spanned<Expr>, ParserError>;

impl<'a> Parser<'a> {
    /// Parses an expression.
    pub fn parse_expression(&mut self, precedence: u8) -> ExprResult {
        let token = self.advance();
        let mut lhs = self.prefix_rule(token)?;
        todo!()
    }

    fn prefix_rule(&mut self, token: Spanned<TokenKind>) -> ExprResult {
        match token.0 {
            TokenKind::Integer(_) | TokenKind::String(_) | TokenKind::True | TokenKind::False => {
                self.parse_literal(token)
            }
            _ => todo!(),
        }
    }

    fn parse_literal(&mut self, current: Spanned<TokenKind>) -> ExprResult {
        Ok((
            match current.0 {
                TokenKind::Integer(i) => Expr::Literal(LiteralKind::Int(i.parse().unwrap())), // Safe to unwrap, value confirmed to be valid integer.
                TokenKind::String(s) => Expr::Literal(LiteralKind::String(s)),
                TokenKind::True => Expr::Literal(LiteralKind::Bool(true)),
                TokenKind::False => Expr::Literal(LiteralKind::Bool(false)),
                _ => unreachable!("parse_literal is only called when `current` is a literal."),
            },
            current.1,
        ))
    }
}
