//! The lexer takes some source string and generates a stream of
//! `TokenKind`'s. A token is any meaningful "word" or "character",
//! in the sense that items akin to whitespace and comments are filtered out.

use std::{iter::Peekable, str::Chars};

use crate::span::{Span, Spanned};
use crate::tokens::TokenKind;

use unicode_xid::UnicodeXID;

fn get_keyword(name: &str) -> TokenKind {
    match name {
        "and" => TokenKind::And,
        "do" => TokenKind::Do,
        "else" => TokenKind::Else,
        "end" => TokenKind::End,
        "false" => TokenKind::False,
        "for" => TokenKind::For,
        "fun" => TokenKind::Fun,
        "if" => TokenKind::If,
        "let" => TokenKind::Let,
        "or" => TokenKind::Or,
        "return" => TokenKind::Return,
        "true" => TokenKind::True,
        "type" => TokenKind::Type,
        "while" => TokenKind::While,
        _ => TokenKind::Ident(name.to_string()),
    }
}

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

    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    fn create_token(&mut self, kind: TokenKind, len: usize) -> Spanned<TokenKind> {
        (kind, Span::from(self.position - len..self.position))
    }

    fn lex_string(&mut self) -> Spanned<TokenKind> {
        let mut value = String::new();

        // Safe to unwrap, && will short-circuit.
        while !self.at_end() && *self.peek().unwrap() != '"' {
            value.push(self.advance().unwrap());
        }

        let len = value.len();
        if self.at_end() {
            return self.create_token(
                TokenKind::Error("Unterminated string literal. Expected closing quote, instead found EoF (End of File)".to_string()), 
                len
            );
        }

        self.advance(); // Consume closing quote
        self.create_token(TokenKind::String(value), len)
    }

    fn lex_number(&mut self, first_char: char) -> Spanned<TokenKind> {
        let mut value = String::from(first_char);

        // Safe to unwrap, && will short-circuit.
        while !self.at_end() && self.peek().unwrap().is_numeric() {
            value.push(self.advance().unwrap()); // Safe to unwrap since not end of input.
        }

        let len = value.len();
        self.create_token(TokenKind::Integer(value), len)
    }

    fn lex_identifier(&mut self, first_char: char) -> Spanned<TokenKind> {
        let mut value = String::from(first_char);

        // Safe to unwrap, && will short-circuit.
        while !self.at_end() && UnicodeXID::is_xid_continue(*self.peek().unwrap()) {
            value.push(self.advance().unwrap())
        }

        let tt = get_keyword(&value);
        self.create_token(tt, value.len())
    }

    fn next_token(&mut self) -> Spanned<TokenKind> {
        if let Some(c) = self.advance() {
            return match c {
                // Punctuation
                '(' => self.create_token(TokenKind::OpenParen, 1),
                ')' => self.create_token(TokenKind::CloseParen, 1),
                '[' => self.create_token(TokenKind::OpenBracket, 1),
                ']' => self.create_token(TokenKind::CloseBracket, 1),
                ',' => self.create_token(TokenKind::Comma, 1),
                ':' => self.create_token(TokenKind::Colon, 1),

                // Operators
                '=' => {
                    if self.consume('=') {
                        self.create_token(TokenKind::EqualEqual, 2)
                    } else {
                        self.create_token(TokenKind::Equal, 1)
                    }
                }
                '!' => {
                    if self.consume('=') {
                        self.create_token(TokenKind::BangEqual, 2)
                    } else {
                        self.create_token(TokenKind::Bang, 1)
                    }
                }
                '>' => {
                    if self.consume('=') {
                        self.create_token(TokenKind::GreaterEqual, 2)
                    } else {
                        self.create_token(TokenKind::Greater, 1)
                    }
                }
                '<' => {
                    if self.consume('=') {
                        self.create_token(TokenKind::LessEqual, 2)
                    } else {
                        self.create_token(TokenKind::Less, 1)
                    }
                }

                '+' => self.create_token(TokenKind::Plus, 1),
                '*' => self.create_token(TokenKind::Star, 1),
                '/' => self.create_token(TokenKind::Slash, 1),

                '-' => {
                    if self.consume('>') {
                        self.create_token(TokenKind::Arrow, 2)
                    } else {
                        self.create_token(TokenKind::Minus, 1)
                    }
                }

                // Literals
                '"' => self.lex_string(),
                c if c.is_numeric() => self.lex_number(c),
                c if UnicodeXID::is_xid_start(c) || c == '_' => self.lex_identifier(c),

                // Whitespace
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.next_token()
                }
                c if c.is_whitespace() => self.next_token(),

                c => self.create_token(TokenKind::Error(format!("Unknown character {c}")), 1),
            };
        }

        self.create_token(TokenKind::EoF, 0)
    }
}
