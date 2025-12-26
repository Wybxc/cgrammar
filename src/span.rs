//! Span utilities.

use std::rc::Rc;

use chumsky::{
    input::{Input, MappedInput},
    span::SimpleSpan,
};
#[cfg(feature = "dbg-pls")]
use dbg_pls::DebugPls;

use crate::{BalancedToken, BalancedTokenSequence};

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
/// Source context information for error reporting.
pub struct SourceContext {
    /// The source file name.
    pub file: Option<Rc<str>>,
    /// The current line number.
    pub line: usize,
    /// The byte offset of the beginning of the line.
    pub bol: usize,
}

/// A source range with context information.
pub type SourceRange = SimpleSpan<usize, SourceContext>;

#[derive(Debug, Clone, PartialEq)]
/// A value with an associated source range.
pub struct Spanned<T> {
    /// The wrapped value.
    pub value: T,
    /// The source range of the value.
    pub range: SourceRange,
}

impl<T> Spanned<T> {
    /// Create a new spanned value.
    pub fn new(value: T, range: SourceRange) -> Self {
        Spanned { value, range }
    }
}

#[cfg(feature = "dbg-pls")]
impl<T> DebugPls for Spanned<T>
where
    T: DebugPls,
{
    fn fmt(&self, f: dbg_pls::Formatter<'_>) {
        self.value.fmt(f)
    }
}

impl<T> Spanned<T> {
    /// Get references to the value and range separately.
    pub fn as_unzipped(&self) -> (&T, &SourceRange) {
        (&self.value, &self.range)
    }
}

/// Token stream input type for the parser.
pub type Tokens<'a> = MappedInput<
    'a,
    BalancedToken,
    SourceRange,
    &'a [Spanned<BalancedToken>],
    fn(&Spanned<BalancedToken>) -> (&BalancedToken, &SourceRange),
>;

impl BalancedTokenSequence {
    /// Convert this token sequence to a parser input.
    pub fn as_input(&self) -> Tokens<'_> {
        self.tokens.as_slice().map(self.eoi.clone(), Spanned::as_unzipped)
    }
}
