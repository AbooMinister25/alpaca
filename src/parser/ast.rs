//! The Abstract Syntax Tree is a tree-like representation of
//! Alpaca's syntax.

use crate::{span::Spanned, tokens::TokenKind};

/// Kinds of literals
#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    /// Integer literal (`10`)
    Int(i64),
    /// Boolean literal (`true`, `false`)
    Bool(bool),
    /// String literal (`"foo"`)
    String(String),
}

/// Type annotations.
#[derive(Debug, PartialEq)]
pub enum Annotation {
    Single(String),
    Tuple(Vec<Annotation>),
    Array(Vec<Annotation>),
    Function {
        arg_types: Vec<Annotation>,
        ret_type: Box<Annotation>,
    },
}

/// An expression is an item that evaluates to some value.
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// Literals (`10`, `"Hi"`)
    Literal(LiteralKind),
    /// Identifiers (`hello`, `foo`, `bar`)
    Ident(String),
    /// Tuples (`(1, 2, 3)`)
    Tuple(Vec<Spanned<Expr>>),
    /// Arrays (`[1, 2, 3]`)
    Array(Vec<Spanned<Expr>>),
    /// An unary operation (`!foo`, `-bar`)
    Unary {
        op: TokenKind,
        rhs: Box<Spanned<Expr>>,
    },
    /// A binary operation (`5 + 5`)
    Binary {
        op: TokenKind,
        lhs: Box<Spanned<Expr>>,
        rhs: Box<Spanned<Expr>>,
    },
    /// A function call (`foo()`)
    Call {
        callee: Box<Spanned<Expr>>,
        args: Vec<Spanned<Expr>>,
    },
    /// A variable assignment (`foo = 10`)
    Assignment {
        name: Box<Spanned<Expr>>,
        value: Box<Spanned<Expr>>,
    },
    /// A block
    ///
    /// `do <code> end`
    Block(Vec<Spanned<Statement>>),
    /// An `if` expression
    ///
    /// `if <expr> do <code> else <code> end`
    If {
        condition: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
        else_: Box<Option<Spanned<Expr>>>,
    },
    /// A for loop
    ///
    /// `for i in it do <code> end`
    For {
        var: Box<Spanned<Expr>>,
        iter: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
    },
    /// A while loop
    ///
    /// `while <expr> do <code> end`
    While {
        expr: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
    },
}

/// A statement is some standalone unit of code which does something, comprised
/// of one or more statements.
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// An expression statement
    Expression(Spanned<Expr>),
    /// A return statement
    ///
    /// `return <expr>`
    Return(Spanned<Expr>),
    /// A `let` variable declaration
    ///
    /// `let <name> = <expr>`
    Let {
        name: Spanned<Expr>,
        value: Spanned<Expr>,
    },
    /// A function declaration
    ///
    /// `fun <name>(<args>) do <expr> end`
    Function {
        name: Spanned<Expr>,
        public: bool,
        params: Vec<String>,
        annotations: Vec<Spanned<Annotation>>,
        return_annotation: Option<Spanned<Annotation>>,
        body: Spanned<Expr>,
    },
}
