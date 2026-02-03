//! Span utilities.

#[cfg(feature = "report")]
use std::collections::HashMap;
use std::ops::Range;

#[cfg(feature = "report")]
use ariadne::Source;
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

/// An identifier for a source context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct ContextId(i32);

impl From<usize> for ContextId {
    fn from(value: usize) -> Self {
        ContextId(value.try_into().expect("Context ID overflow"))
    }
}

impl ContextId {
    /// A sentinel value representing no context.
    pub const fn none() -> Self {
        ContextId(-1)
    }

    /// Get the index of this context ID, if it is valid.
    pub fn idx(self) -> Option<usize> {
        if self.0 < 0 { None } else { Some(self.0 as usize) }
    }
}

/// A collection of span contexts for tracking source file information.
#[derive(Clone)]
pub struct ContextMapping<'a> {
    /// The original source code.
    pub source: &'a str,
    contexts: Slab<SourceContext>,
    #[cfg(feature = "report")]
    ctx_source: HashMap<ContextId, Source<&'a str>>,
}

impl<'a> ContextMapping<'a> {
    /// Creates a new empty span context collection.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            contexts: Slab::new(),
            #[cfg(feature = "report")]
            ctx_source: HashMap::new(),
        }
    }

    /// Inserts a new source context and returns its ID.
    pub fn insert_context(&mut self, context: SourceContext) -> ContextId {
        self.contexts.insert(context).into()
    }
}

#[cfg(feature = "report")]
impl<'a> ariadne::Cache<ContextId> for ContextMapping<'a> {
    type Storage = &'a str;

    fn fetch(&mut self, id: &ContextId) -> Result<&Source<&'a str>, impl std::fmt::Debug> {
        let source = self.ctx_source.entry(*id).or_insert_with(|| Source::from(self.source));
        Ok::<_, ()>(source)
    }

    fn display<'b>(&self, id: &'b ContextId) -> Option<impl std::fmt::Display + 'b> {
        id.idx()
            .and_then(|idx| self.contexts.get(idx))
            .map(|ctx| ctx.filename.to_string())
    }
}

/// A source span.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
pub struct Span {
    start: usize,
    len: u32,
    ctx_id: ContextId,
}

impl Default for Span {
    fn default() -> Self {
        Self {
            start: 0,
            len: 0,
            ctx_id: ContextId::none(),
        }
    }
}

impl Span {
    /// Create a new span from a range and context ID.
    pub fn new(range: std::ops::Range<usize>, ctx_id: ContextId) -> Self {
        Self {
            start: range.start,
            len: range.len().try_into().expect("Span length overflow"),
            ctx_id,
        }
    }

    /// Create a new end-of-input span at the given position and context ID.
    pub fn new_eoi(pos: usize, ctx_id: ContextId) -> Self {
        Self { start: pos, len: 0, ctx_id }
    }

    /// Get the context and range of this span.
    pub fn at_context<'a>(self, ctx_map: &'a ContextMapping) -> (Option<&'a SourceContext>, Range<usize>) {
        let context = self.ctx_id.idx().and_then(|id: usize| ctx_map.contexts.get(id));
        let range = self.start..(self.start + self.len as usize);
        (context, range)
    }
}

impl chumsky::span::Span for Span {
    type Context = ContextId;

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

#[cfg(feature = "report")]
impl ariadne::Span for Span {
    type SourceId = ContextId;

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

    /// Create a new spanned value with a default span.
    pub fn dummy(value: T) -> Self {
        Spanned { value, span: Span::default() }
    }

    /// Map the value to a new type.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned { value: f(self.value), span: self.span }
    }

    /// Map the value to a new type, keeping the span.
    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned { value: &self.value, span: self.span }
    }

    /// Get the inner value, discarding the span.
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Get a reference pair of value and span.
    pub fn as_pair(&self) -> (&T, &Span) {
        (&self.value, &self.span)
    }
}

impl<T: Default> Default for Spanned<T> {
    fn default() -> Self {
        Self::dummy(T::default())
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
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
        self.tokens.as_slice().map(self.eoi, Spanned::as_pair)
    }
}
