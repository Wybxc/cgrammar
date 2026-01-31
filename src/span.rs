//! Span utilities.

use std::ops::Range;

use chumsky::input::{Input, MappedInput};
#[cfg(feature = "dbg-pls")]
use dbg_pls::DebugPls;

use crate::{BalancedToken, BalancedTokenSequence, utils::Slab};

/// Source context information for error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct SourceContext {
    /// The original filename.
    pub filename: String,
    /// The offset from the line numbers in the input code to the original source file.
    pub line_offset: i32,
}

/// A collection of span contexts for tracking source file information.
#[derive(Clone)]
pub struct ContextMapping {
    contexts: Slab<SourceContext>,
}

impl Default for ContextMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextMapping {
    /// Creates a new empty span context collection.
    pub fn new() -> Self {
        Self { contexts: Slab::new() }
    }

    /// Inserts a new source context and returns its ID.
    pub fn insert_context(&mut self, context: SourceContext) -> i32 {
        let id = self.contexts.insert(context);
        id.try_into().expect("Source context ID overflow")
    }
}

/// A source span.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    len: u32,
    ctx_id: i32,
}

impl Default for Span {
    fn default() -> Self {
        Self { start: 0, len: 0, ctx_id: -1 }
    }
}

impl Span {
    /// Create a new span from a range and context ID.
    pub fn new(range: std::ops::Range<usize>, ctx_id: i32) -> Self {
        Self {
            start: range.start,
            len: range.len().try_into().expect("Span length overflow"),
            ctx_id,
        }
    }

    /// Create a new end-of-input span at the given position and context ID.
    pub fn new_eoi(pos: usize, ctx_id: i32) -> Self {
        Self { start: pos, len: 0, ctx_id }
    }

    /// Get the context and range of this span.
    pub fn at_context(self, ctx_map: &ContextMapping) -> (Option<&SourceContext>, Range<usize>) {
        let context = self
            .ctx_id
            .try_into()
            .ok()
            .and_then(|id: usize| ctx_map.contexts.get(id));
        let range = self.start..(self.start + self.len as usize);
        (context, range)
    }
}

impl chumsky::span::Span for Span {
    type Context = i32;

    type Offset = usize;

    fn new(context: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        Self {
            start: range.start,
            len: range.len().try_into().expect("Span length overflow"),
            ctx_id: context,
        }
    }

    fn context(&self) -> Self::Context {
        self.ctx_id
    }

    fn start(&self) -> Self::Offset {
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.start + self.len as usize
    }
}

impl ariadne::Span for Span {
    type SourceId = i32;

    fn source(&self) -> &Self::SourceId {
        &self.ctx_id
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.start + self.len as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A value with an associated source span.
pub struct Spanned<T> {
    /// The wrapped value.
    pub value: T,
    /// The source span of the value.
    pub span: Span,
}

impl<T> Spanned<T> {
    /// Create a new spanned value.
    pub fn new(value: T, span: Span) -> Self {
        Spanned { value, span }
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
    pub fn as_ref(&self) -> (&T, &Span) {
        (&self.value, &self.span)
    }
}

/// Token stream input type for the parser.
pub type Tokens<'a> = MappedInput<
    'a,
    BalancedToken,
    Span,
    &'a [Spanned<BalancedToken>],
    fn(&Spanned<BalancedToken>) -> (&BalancedToken, &Span),
>;

impl BalancedTokenSequence {
    /// Convert this token sequence to a parser input.
    pub fn as_input(&self) -> Tokens<'_> {
        self.tokens.as_slice().map(self.eoi, Spanned::as_ref)
    }
}
