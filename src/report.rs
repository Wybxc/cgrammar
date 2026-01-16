//! Error reporting.

use std::fmt;

use crate::{ast::*, parser_utils::Error, span::SpanContexts};

struct DiagnosticToken(BalancedToken);

impl fmt::Display for DiagnosticToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            BalancedToken::Parenthesized(_) => write!(f, "(...)"),
            BalancedToken::Bracketed(_) => write!(f, "[...]"),
            BalancedToken::Braced(_) => write!(f, "{{...}}"),
            BalancedToken::Identifier(_) => write!(f, "identifier"),
            BalancedToken::StringLiteral(_) => write!(f, "string literal"),
            BalancedToken::QuotedString(_) => write!(f, "quoted string"),
            BalancedToken::Constant(_) => write!(f, "constant"),
            BalancedToken::Punctuator(_) => write!(f, "punctuator"),
            #[cfg(feature = "quasi-quote")]
            BalancedToken::Interpolation(_) => write!(f, "interpolation"),
            BalancedToken::Unknown => write!(f, "unknown token"),
        }
    }
}

/// Report an error to stderr.
pub fn report<'a>(error: Error<'a>, contexts: &SpanContexts) {
    let error = error.map_token(DiagnosticToken);
    let span = error.span();
    let file = span.context.filename(contexts).unwrap_or("<unknown>");
    let line = span.context.line;
    eprintln!("error[{}:{}] {}", file, line, error.reason());
}
