use std::{
    collections::hash_map::Entry,
    fmt::{self, Display},
    hash::Hash,
};

use crate::{ast::*, span::*};

use ariadne::{Cache, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use rustc_hash::FxHashMap;

pub type Error<'a> = Rich<'a, BalancedToken, SourceRange>;

pub struct Span(SourceRange);

impl ariadne::Span for Span {
    type SourceId = str;

    fn source(&self) -> &Self::SourceId {
        self.0.context.file.as_deref().unwrap_or_default()
    }

    fn start(&self) -> usize {
        self.0.start
    }

    fn end(&self) -> usize {
        self.0.end
    }
}

struct DiagnosticToken(BalancedToken);

impl Display for DiagnosticToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            BalancedToken::Parenthesized(_) => write!(f, "(...)"),
            BalancedToken::Bracketed(_) => write!(f, "[...]"),
            BalancedToken::Braced(_) => write!(f, "{{...}}"),
            BalancedToken::Identifier(_) => write!(f, "identifier"),
            BalancedToken::StringLiteral(_) => write!(f, "string literal"),
            BalancedToken::Constant(_) => write!(f, "constant"),
            BalancedToken::Punctuator(_) => write!(f, "punctuator"),
            BalancedToken::Unknown => todo!(),
        }
    }
}

pub fn report(error: Error) -> Report<Span> {
    let error = error.map_token(DiagnosticToken);
    Report::build(ReportKind::Error, Span(error.span().clone()))
        .with_label(Label::new(Span(error.span().clone())).with_message(error.reason().to_string()))
        .with_message("syntax error")
        .finish()
}

#[derive(Debug, Clone)]
pub struct FnCache<Id, F, I>
where
    I: AsRef<str>,
    Id: ToOwned + ?Sized,
{
    sources: FxHashMap<Id::Owned, Source<I>>,
    get: F,
}

impl<Id, F, I> FnCache<Id, F, I>
where
    I: AsRef<str>,
    Id: ToOwned + ?Sized,
{
    /// Create a new [`FnCache`] with the given fetch function.
    pub fn new(get: F) -> Self {
        Self { sources: FxHashMap::default(), get }
    }
}

impl<Id: fmt::Display, F, I, E> Cache<Id> for FnCache<Id, F, I>
where
    I: AsRef<str>,
    Id: ToOwned + ?Sized,
    Id::Owned: Hash + Eq,
    E: fmt::Debug,
    F: for<'a> FnMut(&'a Id) -> Result<I, E>,
{
    type Storage = I;

    fn fetch(&mut self, id: &Id) -> Result<&Source<I>, impl fmt::Debug> {
        Ok::<_, E>(match self.sources.entry(id.to_owned()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(Source::from((self.get)(id)?)),
        })
    }

    fn display<'a>(&self, id: &'a Id) -> Option<impl fmt::Display + 'a> {
        Some(Box::new(id))
    }
}
