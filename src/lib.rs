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
