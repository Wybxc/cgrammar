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
/// Returns the tree and the typed AST. If parsing fails, the tree still
/// contains all tokens consumed before the failure.
pub fn parse_tree(source: &str) -> (SyntaxTree, Option<TranslationUnit>) {
    parse_tree_with_typedefs(source, &[])
}

/// Parse C source code with initial typedef names registered.
/// C parsers need typedef names to distinguish type names from identifiers
/// in declarations.
pub fn parse_tree_with_typedefs(source: &str, typedefs: &[&str]) -> (SyntaxTree, Option<TranslationUnit>) {
    let (tokens, _ctx_map) = lex(source, None);
    let mut state = ParseState::new();
    for name in typedefs {
        state.ctx_mut().add_typedef_name((*name).into());
    }
    state.green.set_source_len(source.len() as u32);
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    let green = state.green.build();
    (SyntaxTree::new(green), result.output().cloned())
}

/// Parse C source code and return the [`SyntaxTree`], [`ContextMapping`]
/// (for [`print_lossless`]), and typed AST. Accepts initial typedef names.
pub fn parse_tree_with_map<'a>(source: &'a str) -> (SyntaxTree, span::ContextMapping<'a>, Option<TranslationUnit>) {
    let (tokens, ctx_map) = lex(source, None);
    let mut state = ParseState::new();
    state.green.set_source_len(source.len() as u32);
    let result = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    let green = state.green.build();
    (SyntaxTree::new(green), ctx_map, result.output().cloned())
}
