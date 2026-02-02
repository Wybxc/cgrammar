#![warn(missing_docs, missing_copy_implementations)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod utils;

mod ast;
mod context;
mod lexer;
pub mod parser;
#[cfg(feature = "printer")]
pub mod printer;
mod report;
pub mod span;
pub mod visitor;

pub use ast::*;
pub use chumsky::Parser;
pub use context::State;
pub use lexer::lex;
pub use parser::*;
pub use report::*;
pub use visitor::{Visitor, VisitorMut};

mod lexer2;

/// Lexer v2 (hand-written version) for testing
pub mod lexer_v2 {
    pub use crate::lexer2::lex;
}
