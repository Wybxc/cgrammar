//! Span utilities.

use chumsky::{
    input::{Input, MappedInput},
    span::SimpleSpan,
};
#[cfg(feature = "dbg-pls")]
use dbg_pls::DebugPls;

use crate::{BalancedToken, BalancedTokenSequence, utils::Slab};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg-pls", derive(DebugPls))]
/// Source context information for error reporting.
pub struct SourceContext {
    /// The source file name.
    pub(crate) file_id: isize,
    /// The current line number.
    pub line: usize,
    /// The byte offset of the beginning of the line.
    pub bol: usize,
}

impl Default for SourceContext {
    fn default() -> Self {
        Self { file_id: -1, line: 0, bol: 0 }
    }
}

impl SourceContext {
    /// Returns the filename associated with this source context.
    pub fn filename<'a>(&self, interner: &'a SpanContexts) -> Option<&'a str> {
        interner.get_filename(self.file_id)
    }
}

/// A collection of span contexts for tracking source file information.
#[derive(Clone)]
pub struct SpanContexts {
    filenames: Slab<String>,
}

impl Default for SpanContexts {
    fn default() -> Self {
        Self::new()
    }
}

impl SpanContexts {
    /// Creates a new empty span context collection.
    pub fn new() -> Self {
        Self { filenames: Slab::new() }
    }

    /// Interns a filename and returns its unique identifier.
    pub fn intern_filename(&mut self, filename: &str) -> isize {
        if let Some(id) = self.filenames.iter().position(|s| s == filename) {
            id as isize
        } else {
            let id = self.filenames.insert(filename.to_string());
            id.try_into().expect("Filename ID overflow")
        }
    }

    /// Retrieves a filename by its interned ID.
    pub fn get_filename(&self, id: isize) -> Option<&str> {
        match usize::try_from(id) {
            Ok(id) => self.filenames.get(id).map(|s| s.as_str()),
            Err(_) => None,
        }
    }
}

/// A source range with context information.
pub type SourceRange = SimpleSpan<usize, SourceContext>;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        self.tokens.as_slice().map(self.eoi, Spanned::as_unzipped)
    }
}
