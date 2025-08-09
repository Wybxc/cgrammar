use std::rc::Rc;

use chumsky::{
    input::{Input, MappedInput},
    span::SimpleSpan,
};
use dbg_pls::DebugPls;

use crate::{BalancedToken, BalancedTokenSequence};

#[derive(Debug, DebugPls, Default, Clone, PartialEq)]
pub struct SourceContext {
    pub file: Option<Rc<str>>,
    pub line: usize,
    pub bol: usize,
}

pub type SourceRange = SimpleSpan<usize, SourceContext>;

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub range: SourceRange,
}

impl<T> Spanned<T> {
    pub fn new(value: T, range: SourceRange) -> Self {
        Spanned { value, range }
    }
}

impl<T> DebugPls for Spanned<T>
where
    T: DebugPls,
{
    fn fmt(&self, f: dbg_pls::Formatter<'_>) {
        self.value.fmt(f)
    }
}

impl<T> Spanned<T> {
    pub fn as_unzipped(&self) -> (&T, &SourceRange) {
        (&self.value, &self.range)
    }
}

pub type Tokens<'a> = MappedInput<
    BalancedToken,
    SourceRange,
    &'a [Spanned<BalancedToken>],
    fn(&Spanned<BalancedToken>) -> (&BalancedToken, &SourceRange),
>;

impl BalancedTokenSequence {
    pub fn as_input(&self) -> Tokens<'_> {
        self.tokens.as_slice().map(self.eoi.clone(), Spanned::as_unzipped)
    }
}
