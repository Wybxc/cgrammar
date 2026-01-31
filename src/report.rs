//! Error reporting.

use std::fmt;

use crate::{ast::*, parser_utils::Error, span::ContextMapping};

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
            BalancedToken::Template(_) => write!(f, "template"),
            #[cfg(feature = "quasi-quote")]
            BalancedToken::Interpolation(_) => write!(f, "interpolation"),
            BalancedToken::Unknown => write!(f, "unknown token"),
        }
    }
}

/// Report an error to stderr.
pub fn report<'a>(error: Error<'a>, ctx_map: &ContextMapping) {
    let error = error.map_token(DiagnosticToken);
    let span = error.span();
    let (ctx, _range) = span.at_context(ctx_map);
    eprintln!(
        "error[{}] {}",
        ctx.map_or("<unknown>", |c| c.filename.as_str()),
        error.reason()
    );
}
