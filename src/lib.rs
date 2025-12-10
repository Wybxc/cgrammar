mod ast;
mod context;
mod lexer;
mod parser;
mod report;
pub mod span;
mod utils;

pub use ast::*;
pub use context::{Context, ContextTweaker};
pub use lexer::*;
pub use parser::*;
pub use report::*;

pub struct CLexer;

impl CLexer {
    pub fn lex<'a>(input: &'a str) -> chumsky::ParseResult<BalancedTokenSequence, chumsky::error::Simple<'a, char>> {
        use chumsky::Parser;
        let lexer = balanced_token_sequence();
        lexer.parse(input)
    }
}

pub struct CParser;

impl CParser {
    pub fn parse<'a>(
        tokens: &'a BalancedTokenSequence,
    ) -> chumsky::ParseResult<TranslationUnit, parser::parser_utils::Error<'a>> {
        use chumsky::Parser;
        let parser = translation_unit::<()>();
        parser.parse(tokens.as_input())
    }
}

pub struct CParserWithTweaker<T: ContextTweaker>(std::marker::PhantomData<T>);

impl<T: ContextTweaker + 'static> CParserWithTweaker<T> {
    pub fn parse<'a>(
        tokens: &'a BalancedTokenSequence,
    ) -> chumsky::ParseResult<TranslationUnit, parser::parser_utils::Error<'a>> {
        use chumsky::Parser;
        let parser = translation_unit::<T>();
        parser.parse(tokens.as_input())
    }
}