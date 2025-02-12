/// Every token in Alpaca.
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Punctuation
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Dot,
    Colon,
    Arrow,

    // Operators
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,

    // Literals
    String(String),
    Integer(String),

    // Identifiers
    Ident(String),

    // Keywords
    And,
    Do,
    Else,
    End,
    False,
    For,
    Fun,
    If,
    Let,
    Or,
    Return,
    True,
    Type,
    While,

    // Misc
    Error(String),
    EoF,
}
