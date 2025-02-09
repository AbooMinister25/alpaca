//! The lexer takes some source string and generates a stream of
//! `TokenKind`'s. A token is any meaningful "word" or "character",
//! in the sense that items akin to whitespace and comments are filtered out.

use std::{iter::Peekable, str::Chars};

use crate::span::{Span, Spanned};
use crate::tokens::TokenKind;

use unicode_xid::UnicodeXID;

/// Generates a stream of `TokenKind`'s from some
/// UTF-8 encoded string.
///
/// Generates tokens on-demand from teh given source string,
/// filtering out unnecessary items.
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    position: usize,
    line: u32,
    column: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            position: 0,
            line: 1,
            column: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.source.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn consume(&mut self, expected: char) -> bool {
        if let Some(c) = self.peek() {
            if c == &expected {
                self.advance();
                return true;
            }
        }

        false
    }
}
