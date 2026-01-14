//! Dump the AST of a C source file.
//!
//! Usage: `cargo run --example shrink_test_case --all-features -- path/to/source.c`

use cgrammar::*;
use chumsky::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseResult {
    Success,
    LexError,
    ParseError,
}

fn can_parse(file: &str, src: &str) -> ParseResult {
    let lexer = balanced_token_sequence();
    let mut lexer_state = LexerState::new(Some(file));
    let tokens = lexer.parse_with_state(src, &mut lexer_state);
    let Some(tokens) = tokens.output() else {
        return ParseResult::LexError;
    };

    let parser = translation_unit();
    let mut init_state = State::new();
    init_state.ctx_mut().add_typedef_name("term".into());
    init_state.ctx_mut().add_typedef_name("thm".into());
    let ast = parser.parse_with_state(tokens.as_input(), &mut init_state);
    if ast.has_errors() {
        ParseResult::ParseError
    } else {
        ParseResult::Success
    }
}

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(file.as_str()).unwrap();

    // shrink 1: trim half the lines from the end
    let mut low = 0;
    let mut high = src.lines().count();
    while low < high {
        let mid = (low + high) / 2;
        eprint!("{}...", mid);
        let truncated_src = src.lines().take(mid).collect::<Vec<_>>().join("\n");
        if can_parse(&file, &truncated_src) == ParseResult::ParseError {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    eprintln!();

    let minimal_src = src.lines().take(low + 1).collect::<Vec<_>>().join("\n");
    println!("{}", minimal_src);
}
