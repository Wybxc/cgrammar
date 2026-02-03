//! Error reporting.

use std::fmt;

#[cfg(feature = "report")]
use crate::span::Span;
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

/// Convert a parse error to an ariadne report for pretty printing.
#[cfg(feature = "report")]
pub fn report_ariadne<'a>(error: Error<'a>) -> ariadne::Report<'a, Span> {
    use ariadne::{Label, Report, ReportKind};
    use chumsky::error::RichReason;

    let error = error.map_token(DiagnosticToken);
    let span = *error.span();

    let message = match error.reason() {
        RichReason::ExpectedFound { expected, found } => {
            let expected_str = if expected.is_empty() {
                "something else".to_string()
            } else {
                expected.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
            };
            let found_str = found
                .as_ref()
                .map(|f| f.to_string())
                .unwrap_or_else(|| "end of input".to_string());
            format!("expected {}, found {}", expected_str, found_str)
        }
        RichReason::Custom(msg) => msg.clone(),
    };

    let mut builder = Report::build(ReportKind::Error, span).with_message(&message);

    builder = builder.with_label(Label::new(span).with_message(&message));

    // Add context information if available
    for (label, ctx_span) in error.contexts() {
        builder = builder.with_label(Label::new(*ctx_span).with_message(format!("in {}", label)));
    }

    builder.finish()
}
