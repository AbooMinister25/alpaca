use crate::parser::ast::{Annotation, Expr, Statement};
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
            TokenKind::Ident(s) => Ok((Expr::Ident(s), token.1)),
            TokenKind::OpenParen => self.parse_grouping(),
            TokenKind::Minus | TokenKind::Bang => self.parse_unary(token),
            TokenKind::OpenBracket => self.parse_array(token),
            TokenKind::Do => self.parse_block(token),
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

    fn parse_grouping(&mut self) -> ExprResult {
        let expr = self.parse_expression(1)?;

        // If next token is a comma, parse as a tuple
        if self.peek().0 == TokenKind::Comma {
            return self.parse_tuple(expr);
        }

        self.consume(&TokenKind::CloseParen)
            .map_err(|e| e.with_help("Expeted to find a closing parenthesis.".to_string()))?;
        Ok(expr)
    }

    fn parse_tuple(&mut self, first: Spanned<Expr>) -> ExprResult {
        let start = first.1.start;
        let mut items = vec![first];

        while self.peek().0 != TokenKind::CloseParen {
            // Consume a comma if we haven't reached the end of the tuple.
            if self.peek().0 != TokenKind::CloseParen {
                self.consume(&TokenKind::Comma)
                    .map_err(|e| e.with_help("Did you forget a comma?".to_string()))?;
            }

            let item = self.parse_expression(1)?;
            items.push(item);
        }

        self.consume(&TokenKind::CloseParen)
            .map_err(|e| e.with_help("Expeted to find a closing parenthesis.".to_string()))?;
        let span = Span::from(start - 1..self.current_token_span.end);
        Ok((Expr::Tuple(items), span))
    }

    fn parse_unary(&mut self, current: Spanned<TokenKind>) -> ExprResult {
        // 8 is the precedence level for the `!` and `-` unary operators.
        let expr = self.parse_expression(8)?;
        let span = Span::from(current.1.start..expr.1.end);

        Ok((
            Expr::Unary {
                op: current.0,
                rhs: Box::new(expr),
            },
            span,
        ))
    }

    fn parse_array(&mut self, current: Spanned<TokenKind>) -> ExprResult {
        let mut items = Vec::new();

        while self.peek().0 != TokenKind::CloseBracket {
            let item = self.parse_expression(1)?;
            items.push(item);

            // Consume a comma if we haven't reached the end of the array.
            if self.peek().0 != TokenKind::CloseBracket {
                self.consume(&TokenKind::Comma)
                    .map_err(|e| e.with_help("Did you forget a comma?".to_string()))?;
            }
        }

        self.consume(&TokenKind::CloseBracket)
            .map_err(|e| e.with_help("Expeted to find a closing bracket.".to_string()))?;

        let span = Span::from(current.1.start..self.current_token_span.end);
        Ok((Expr::Array(items), span))
    }

    fn parse_block(&mut self, current: Spanned<TokenKind>) -> ExprResult {
        let mut expressions = vec![];

        while !self.at_end() && self.peek().0 != TokenKind::End {
            expressions.push(todo!());
        }

        self.consume(&TokenKind::End)
            .map_err(|e| e.with_help("Did you forget an `end`?".to_string()))?;
        let span = Span::from(current.1.start..self.current_token_span.end);
        Ok((Expr::Block(expressions), span))
    }
}
