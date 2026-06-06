#![warn(missing_docs, missing_copy_implementations)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod utils;

mod ast;
mod context;
pub mod green;
mod lexer;
pub mod parser;
#[cfg(feature = "printer")]
pub mod printer;
pub mod red;
#[cfg(feature = "report")]
mod report;
pub mod span;
pub mod syntax;
pub mod visitor;

pub use ast::*;
pub use chumsky::Parser;
pub use context::{ParseState, State};
pub use green::GreenNode;
pub use lexer::lex;
pub use parser::*;
pub use red::{print_lossless, SyntaxTree, TreeVisitor};
pub use syntax::SyntaxKind;
#[cfg(feature = "report")]
pub use report::*;
pub use visitor::{Visitor, VisitorMut};

/// Parse C source code into a lossless [`SyntaxTree`].
///
/// This is the primary entry point for obtaining a red-green tree. It combines
/// lexing, parsing, and green tree construction into a single call.
///
/// # Example
///
/// ```ignore
/// let tree = cgrammar::parse_tree("int x;");
/// let root = tree.root();
/// assert_eq!(tree.kind(root), cgrammar::SyntaxKind::TranslationUnit);
/// ```
pub fn parse_tree(source: &str) -> (SyntaxTree, TranslationUnit) {
    let (tokens, _ctx_map) = lex(source, None);
    let mut state = ParseState::new();
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    let ast = result.output().cloned().expect("parse should produce output");
    let green = state.green.build();
    (SyntaxTree::new(green), ast)
}

/// Parse C source code and return the [`SyntaxTree`] and [`ContextMapping`]
/// (for source text access needed by [`print_lossless`]).
pub fn parse_tree_with_map<'a>(source: &'a str) -> (SyntaxTree, span::ContextMapping<'a>, TranslationUnit) {
    let (tokens, ctx_map) = lex(source, None);
    let mut state = ParseState::new();
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    let ast = result.output().cloned().expect("parse should produce output");
    let green = state.green.build();
    (SyntaxTree::new(green), ctx_map, ast)
}
