//! The parser takes a string an outputs an Abstract Syntax Tree (AST).
//! Alpaca's parser is implemented as a Pratt parser.

mod ast;
mod expression;
mod statement;

use crate::lexer::Lexer;
use crate::span::{Span, Spanned};
use crate::tokens::TokenKind;

/// Represents the different types of errors the parser
/// may encounter.
#[derive(Debug)]
pub enum ErrorKind {
    /// Expected one of the given items, found something else.
    Expected(Vec<TokenKind>, TokenKind, Span),
    /// An unclosed delimeter.
    Unclosed(TokenKind, Span),
    /// Found an unexpected token.
    Unexpected(TokenKind, Span),
    /// Another type of error occurred with the given message.
    Other(String, Span),
}

/// Parser error.
#[derive(Debug)]
pub struct ParserError {
    kind: ErrorKind,
    help: Option<String>,
}

impl ParserError {
    pub fn new(kind: ErrorKind, help: Option<String>) -> Self {
        Self { kind, help }
    }

    pub fn with_help(self, help: String) -> Self {
        Self {
            kind: self.kind,
            help: Some(help),
        }
    }
}

/// Parses a string into an Abstract Syntax Tree (AST)
pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    filename: &'a str,
    current_token_span: Span,
    peeked: Option<Spanned<TokenKind>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, filename: &'a str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            filename,
            current_token_span: Span::from(0..0),
            peeked: None,
        }
    }

    fn advance(&mut self) -> Spanned<TokenKind> {
        // If a token has been peeked, return that. Otherwise, advance the lexer
        // and return the next token.
        if let Some(t) = self.peeked.take() {
            self.current_token_span = t.1;
            t
        } else {
            let t = self.lexer.next_token();
            self.current_token_span = t.1;
            t
        }
    }

    fn peek(&mut self) -> &Spanned<TokenKind> {
        // If nothing has been peeked, advance and store that token as the peeked value.
        if self.peeked.is_none() {
            self.peeked = Some(self.advance());
        }

        self.peeked.as_ref().unwrap()
    }

    fn at_end(&mut self) -> bool {
        self.peek().0 == TokenKind::EoF
    }

    fn consume(&mut self, expected: &TokenKind) -> Result<(), ParserError> {
        let token = self.peek();

        if token.0 == *expected {
            self.advance(); // Next token was the expected one, so advance.
            return Ok(());
        }

        Err(ParserError::new(
            ErrorKind::Expected(vec![expected.clone()], token.0.clone(), token.1),
            None,
        ))
    }

    fn synchronize(&mut self) {
        while !self.at_end() {
            match self.peek().0 {
                TokenKind::Fun
                | TokenKind::Let
                | TokenKind::Return
                | TokenKind::If
                | TokenKind::For
                | TokenKind::While => break,
                _ => self.advance(),
            };
        }
    }
}
