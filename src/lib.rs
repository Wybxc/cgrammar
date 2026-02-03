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
#[cfg(feature = "report")]
mod report;
pub mod span;
pub mod visitor;

pub use ast::*;
pub use chumsky::Parser;
pub use context::State;
pub use lexer::lex;
pub use parser::*;
#[cfg(feature = "report")]
pub use report::*;
pub use visitor::{Visitor, VisitorMut};
