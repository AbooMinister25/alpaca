//! Contains the `Span` struct and `Spanned` type for representing positions
//! of items in alpaca throughout the source code.

use std::ops::Range;

pub type Spanned<T> = (T, Span);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
