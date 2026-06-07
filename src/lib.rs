#![warn(missing_docs, missing_copy_implementations)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod utils;

mod astrold;
mod context;
pub mod green;
mod lexer;
pub mod parser;

pub mod red;
pub mod span;
pub mod syntax;
pub mod token;

pub use chumsky::Parser;
pub use context::{ParseState, State};
pub use green::GreenNode;
pub use lexer::lex;
pub use parser::*;
pub use red::{SyntaxNode, SyntaxTree, TreeVisitor, print_lossless};
pub use syntax::SyntaxKind;
pub use token::*;

#[cfg(feature = "quasi-quote")]
pub use token::quasi_quote;

/// Parse C source code into a lossless [`SyntaxTree`].
///
/// If parsing fails, the tree still contains all tokens consumed before the
/// failure. Use [`print_lossless`] to reconstruct the original source text.
pub fn parse_tree(source: &str) -> SyntaxTree {
    parse_tree_with_typedefs(source, &[])
}

/// Parse C source code with initial typedef names registered.
pub fn parse_tree_with_typedefs(source: &str, typedefs: &[&str]) -> SyntaxTree {
    let (tokens, _ctx_map) = lex(source, None);
    let mut state = ParseState::new();
    for name in typedefs {
        state.ctx_mut().add_typedef_name((*name).into());
    }
    state.green.set_source_len(source.len() as u32);
    let _ = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    let green = state.green.build();
    SyntaxTree::new(green)
}

/// Parse C source code and return the [`SyntaxTree`] and [`ContextMapping`]
/// (for [`print_lossless`]).
pub fn parse_tree_with_map<'a>(source: &'a str) -> (SyntaxTree, span::ContextMapping<'a>) {
    let (tokens, ctx_map) = lex(source, None);
    let mut state = ParseState::new();
    state.green.set_source_len(source.len() as u32);
    let _ = translation_unit().parse_with_state(tokens.as_input(), &mut state);
    let green = state.green.build();
    (SyntaxTree::new(green), ctx_map)
}
